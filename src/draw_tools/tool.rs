use super::FractalLineTool;
use super::SelectionTool;
use crate::render_objects::RenderObject;
use druid::{im::Vector, Data, Event, EventCtx};
use druid_enums::Matcher;
use trait_enum::trait_enum;

pub trait Tool {
	fn enable(&mut self, data: &mut Vector<RenderObject>);
	fn disable(&mut self, data: &mut Vector<RenderObject>);
	fn event(&mut self, event: &Event, ctx: &mut EventCtx, data: &mut Vector<RenderObject>);

	fn get_preview(&self) -> Option<RenderObject>;

	fn paint(&self, ctx: &mut druid::PaintCtx, env: &druid::Env) {
		if let Some(robj) = self.get_preview() {
			robj.paint(ctx, env)
		}
	}
}

trait_enum! {
	#[derive(Data, Clone, Copy, PartialEq, Matcher)]
	pub enum ToolObj : Tool {
		FractalLineTool,
		SelectionTool,
	}
}
