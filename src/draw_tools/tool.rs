use crate::{renderobject::RenderObject, GraphicsData};
use druid::{Event, EventCtx};

pub trait Tool {
	fn enable(&mut self, data: &mut GraphicsData);
	fn disable(&mut self, data: &mut GraphicsData);

	fn event(&mut self, event: &Event, ctx: &mut EventCtx, data: &mut GraphicsData);

	fn get_preview(&self) -> Option<RenderObject>;
}
