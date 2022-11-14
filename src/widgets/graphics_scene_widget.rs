use druid::im::OrdSet;
use druid::widget::ClipBox;
use druid::widget::Controller;
use druid::widget::ControllerHost;
use druid::Color;
use druid::Event;
use druid::Point;
use druid::RenderContext;
use druid::Widget;
use druid::WidgetExt;

use super::graphics_data::GraphicsData;

pub struct GraphicsWidget;

#[derive(PartialEq)]
enum GraphicsControllerState {
	Passthrough,
	Panning(Point),
}
// Mainly exists to handle panning the viewport
pub struct GraphicsWidgetController {
	state: GraphicsControllerState,
}

impl Controller<GraphicsData, ClipBox<GraphicsData, GraphicsWidget>> for GraphicsWidgetController {
	fn event(
		&mut self,
		child: &mut ClipBox<GraphicsData, GraphicsWidget>,
		ctx: &mut druid::EventCtx,
		event: &druid::Event,
		data: &mut GraphicsData,
		env: &druid::Env,
	) {
		if let Event::MouseDown(e) = event {
			if e.button.is_middle() {
				ctx.set_handled();
				self.state = GraphicsControllerState::Panning(e.pos);
			}
		}
		if let GraphicsControllerState::Panning(origin) = self.state {
			match event {
				Event::MouseUp(e) => {
					if e.button.is_middle() {
						ctx.set_handled();
						self.state = GraphicsControllerState::Passthrough;
					}
				}
				Event::MouseMove(e) => {
					ctx.set_handled();
					child.pan_by(origin - e.pos);
				}
				_ => (),
			}
		}
		child.event(ctx, event, data, env);
	}
}

impl GraphicsWidget {
	pub fn construct_full(
	) -> ControllerHost<ClipBox<GraphicsData, GraphicsWidget>, GraphicsWidgetController> {
		ClipBox::new(GraphicsWidget).controller(GraphicsWidgetController {
			state: GraphicsControllerState::Passthrough,
		})
	}
}

impl Widget<GraphicsData> for GraphicsWidget {
	fn event(
		&mut self,
		ctx: &mut druid::EventCtx,
		event: &druid::Event,
		data: &mut GraphicsData,
		_env: &druid::Env,
	) {
		data.tool.event(event, ctx, &mut data.objects);
		if !ctx.is_handled() {
			#[allow(clippy::single_match)]
			// We expect to match other expressions later, but this is the only one that matters now
			match event {
				druid::Event::WindowSize(_) => {
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
		for diff in old_data.objects.diff(&data.objects) {
			match diff {
				druid::im::ordset::DiffItem::Remove(item)
				| druid::im::ordset::DiffItem::Add(item) => {
					ctx.request_paint_rect(item.get_drawable().AABB());
				}
				druid::im::ordset::DiffItem::Update { old, new } => {
					ctx.request_paint_rect(new.get_drawable().AABB());
					ctx.request_paint_rect(old.get_drawable().AABB());
				}
			}
		}

		if old_data.tool != data.tool {
			if let Some(robj) = old_data.tool.get_preview() {
				ctx.request_paint_rect(robj.get_drawable().AABB());
			}
			if let Some(robj) = data.tool.get_preview() {
				ctx.request_paint_rect(robj.get_drawable().AABB());
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
		bc.max()
	}

	fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &GraphicsData, env: &druid::Env) {
		let mut redraw_needed = OrdSet::new();
		for object in data.objects.iter().cloned() {
			if !object
				.get_drawable()
				.AABB()
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
