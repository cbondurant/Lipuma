use crate::render_objects::RenderObject;
use druid::{im::OrdSet, Event, EventCtx};

pub trait Tool {
	fn enable(&mut self, data: &mut OrdSet<RenderObject>);
	fn disable(&mut self, data: &mut OrdSet<RenderObject>);
	fn event(
		&mut self,
		event: &Event,
		ctx: &mut EventCtx,
		data: OrdSet<RenderObject>,
	) -> OrdSet<RenderObject>;

	fn get_preview(&self) -> Option<RenderObject>;
}
