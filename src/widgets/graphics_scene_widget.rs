use druid::im::OrdSet;
use druid::Color;
use druid::RenderContext;
use druid::Widget;

use super::graphics_data::GraphicsData;

pub struct GraphicsWidget {}

impl GraphicsWidget {}

impl Widget<GraphicsData> for GraphicsWidget {
	fn event(
		&mut self,
		ctx: &mut druid::EventCtx,
		event: &druid::Event,
		data: &mut GraphicsData,
		_env: &druid::Env,
	) {
		data.objects = data.tool.event(event, ctx, data.objects.clone());
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
