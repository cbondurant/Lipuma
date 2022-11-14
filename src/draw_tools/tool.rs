use super::fractal_line_tool::FractalLineTool;
use super::selection_tool::SelectionTool;
use crate::render_objects::RenderObject;
use druid::{im::OrdSet, Data, Event, EventCtx};
use trait_enum::trait_enum;

pub trait Tool {
	fn enable(&mut self, data: &mut OrdSet<RenderObject>);
	fn disable(&mut self, data: &mut OrdSet<RenderObject>);
	fn event(&mut self, event: &Event, ctx: &mut EventCtx, data: &mut OrdSet<RenderObject>);

	fn get_preview(&self) -> Option<RenderObject>;

	fn paint(&self, ctx: &mut druid::PaintCtx, env: &druid::Env) {
		if let Some(robj) = self.get_preview() {
			robj.paint(ctx, env)
		}
	}
}

trait_enum! {
	#[derive(Data, Clone, Copy, PartialEq)]
	pub enum ToolObj : Tool {
		FractalLineTool,
		SelectionTool
	}
}
