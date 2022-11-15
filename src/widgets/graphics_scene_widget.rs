use druid::im::OrdSet;
use druid::Affine;
use druid::Color;
use druid::Event;
use druid::Point;
use druid::Rect;
use druid::RenderContext;
use druid::Size;
use druid::Widget;

use super::graphics_data::GraphicsData;

#[allow(dead_code)] // Dead code allowed for when we decide to add scaling and rotation gestures
enum GraphicsWidgetState {
	Standby,
	Panning(Point),
	Scaling(Point, Point),
	Rotating(Point, Point),
}

pub struct GraphicsWidget {
	port: Rect,
	state: GraphicsWidgetState,
}

impl GraphicsWidget {
	pub fn new() -> Self {
		Self {
			port: Default::default(),
			state: GraphicsWidgetState::Standby,
		}
	}

	fn handle_transformation_events(
		&mut self,
		ctx: &mut druid::EventCtx,
		event: &Event,
		data: &mut GraphicsData,
		_env: &druid::Env,
	) {
		match event {
			Event::MouseDown(e) => {
				if e.button.is_middle() {
					self.state = GraphicsWidgetState::Panning(e.pos);
					ctx.set_handled();
				}
			}
			Event::MouseMove(e) => match self.state {
				GraphicsWidgetState::Standby => (),
				GraphicsWidgetState::Panning(p) => {
					data.transform *= Affine::translate(
						(data.get_rot_scale().inverse() * (e.pos - p).to_point()).to_vec2(),
					);
					self.state = GraphicsWidgetState::Panning(e.pos);
					ctx.set_handled();
				}
				GraphicsWidgetState::Scaling(_, _) => todo!(),
				GraphicsWidgetState::Rotating(_, _) => todo!(),
			},
			Event::MouseUp(e) => {
				if e.button.is_middle() {
					self.state = GraphicsWidgetState::Standby;
					ctx.set_handled();
				}
			}
			Event::Wheel(e) => match e.wheel_delta.y.partial_cmp(&0.0) {
				Some(cmp) => match cmp {
					std::cmp::Ordering::Less => {
						data.scale_around_point((data.transform.inverse() * e.pos).to_vec2(), 1.01)
					}
					std::cmp::Ordering::Equal => (),
					std::cmp::Ordering::Greater => data.scale_around_point(
						(data.transform.inverse() * e.pos).to_vec2(),
						1.0 / 1.01,
					),
				},
				None => todo!(),
			},
			Event::Zoom(s) => {
				data.transform *= Affine::scale(*s);
			}
			_ => (),
		}
	}

	pub fn adjust_event_by_transform(e: Event, trans: Affine) -> Event {
		match e {
			Event::MouseDown(mut e) => {
				e.pos = trans * e.pos;
				Event::MouseDown(e)
			}
			Event::MouseMove(mut e) => {
				e.pos = trans * e.pos;
				Event::MouseMove(e)
			}
			Event::MouseUp(mut e) => {
				e.pos = trans * e.pos;
				Event::MouseUp(e)
			}
			Event::Wheel(mut e) => {
				e.pos = trans * e.pos;
				Event::Wheel(e)
			}
			// Only mouse events have positions to modify
			non_mouse => non_mouse,
		}
	}

	pub fn get_offset_to_center_as_affine(&self) -> Affine {
		Affine::translate((self.port.size() / 2.0).to_vec2())
	}
}

impl Widget<GraphicsData> for GraphicsWidget {
	fn event(
		&mut self,
		ctx: &mut druid::EventCtx,
		event: &Event,
		data: &mut GraphicsData,
		env: &druid::Env,
	) {
		self.handle_transformation_events(ctx, event, data, env);
		if ctx.is_handled() {
			return;
		}
		let trans_event =
			&Self::adjust_event_by_transform(event.clone(), data.get_trans_to_widget().inverse());
		data.tool.event(trans_event, ctx, &mut data.objects);
		if !ctx.is_handled() {
			#[allow(clippy::single_match)]
			// We expect to match other expressions later, but this is the only one that matters now
			match event {
				Event::WindowSize(_) => {
					// Need to request full repaint to ensure everything draws correctly
					ctx.request_paint();
				}
				_ => (),
			}
		}
		data.preview = data.tool.get_preview();
	}

	fn lifecycle(
		&mut self,
		_ctx: &mut druid::LifeCycleCtx,
		_event: &druid::LifeCycle,
		_data: &GraphicsData,
		_env: &druid::Env,
	) {
	}

	fn update(
		&mut self,
		ctx: &mut druid::UpdateCtx,
		old_data: &GraphicsData,
		data: &GraphicsData,
		_env: &druid::Env,
	) {
		let old_to_widget = old_data.get_trans_to_widget();
		let to_widget = data.get_trans_to_widget();
		if old_to_widget != to_widget {
			ctx.request_paint();
			return;
		}

		for diff in old_data.objects.diff(&data.objects) {
			match diff {
				druid::im::ordset::DiffItem::Remove(item)
				| druid::im::ordset::DiffItem::Add(item) => {
					ctx.request_paint_rect(
						to_widget.transform_rect_bbox(item.get_drawable().AABB()),
					);
				}
				druid::im::ordset::DiffItem::Update { old, new } => {
					ctx.request_paint_rect(
						to_widget.transform_rect_bbox(old.get_drawable().AABB()),
					);
					ctx.request_paint_rect(
						to_widget.transform_rect_bbox(new.get_drawable().AABB()),
					);
				}
			}
		}

		if old_data.tool != data.tool {
			if let Some(item) = old_data.tool.get_preview() {
				ctx.request_paint_rect(to_widget.transform_rect_bbox(item.get_drawable().AABB()));
			}
			if let Some(item) = data.tool.get_preview() {
				ctx.request_paint_rect(to_widget.transform_rect_bbox(item.get_drawable().AABB()));
			}
		}
	}

	fn layout(
		&mut self,
		_ctx: &mut druid::LayoutCtx,
		bc: &druid::BoxConstraints,
		_data: &GraphicsData,
		_env: &druid::Env,
	) -> druid::Size {
		self.port = bc
			.constrain(Size::new(f64::INFINITY, f64::INFINITY))
			.to_rect();
		self.port.size()
	}

	fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &GraphicsData, env: &druid::Env) {
		// For some reason without this we end up clipping WAY out of bounds on full redraw
		ctx.clip(self.port);

		// Need this for multiple operations, so reduce calls
		let to_widget_space = data.get_trans_to_widget();

		// Transform our entire draw context into widget-space
		ctx.transform(to_widget_space);

		let mut redraw_needed = OrdSet::new();
		for object in data.objects.iter() {
			if !to_widget_space
				.transform_rect_bbox(object.get_drawable().AABB())
				.intersect(ctx.region().bounding_box())
				.is_empty()
			{
				redraw_needed.insert(object);
			}
		}

		ctx.clear(Color::WHITE);

		ctx.save().unwrap();
		for robj in redraw_needed {
			robj.paint(ctx, env);
		}
		data.tool.paint(ctx, env);
		ctx.restore().unwrap();
	}
}
