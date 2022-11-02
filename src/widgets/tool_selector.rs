use super::graphics_data::GraphicsData;
use druid::Widget;

struct ToolSelector {}

impl Widget<GraphicsData> for ToolSelector {
	fn event(
		&mut self,
		ctx: &mut druid::EventCtx,
		event: &druid::Event,
		data: &mut GraphicsData,
		env: &druid::Env,
	) {
		todo!()
	}

	fn lifecycle(
		&mut self,
		ctx: &mut druid::LifeCycleCtx,
		event: &druid::LifeCycle,
		data: &GraphicsData,
		env: &druid::Env,
	) {
		todo!()
	}

	fn update(
		&mut self,
		ctx: &mut druid::UpdateCtx,
		old_data: &GraphicsData,
		data: &GraphicsData,
		env: &druid::Env,
	) {
		todo!()
	}

	fn layout(
		&mut self,
		ctx: &mut druid::LayoutCtx,
		bc: &druid::BoxConstraints,
		data: &GraphicsData,
		env: &druid::Env,
	) -> druid::Size {
		todo!()
	}

	fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &GraphicsData, env: &druid::Env) {
		todo!()
	}
}
