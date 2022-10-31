use crate::{render_objects::RenderObject, GraphicsData};
use druid::{im::OrdSet, Event, EventCtx};
use dyn_clonable::*;

#[clonable]
pub trait Tool: Clone {
	fn enable(&mut self, data: &mut GraphicsData);
	fn disable(&mut self, data: &mut GraphicsData);

	fn event(
		&mut self,
		event: &Event,
		ctx: &mut EventCtx,
		data: OrdSet<RenderObject>,
	) -> OrdSet<RenderObject>;

	fn get_preview(&self) -> Option<RenderObject>;
}
