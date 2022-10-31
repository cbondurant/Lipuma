use crate::draw_tools::tool::Tool;
use crate::render_objects::RenderObject;
use druid::im::OrdSet;
use druid::Data;
use std::rc::Rc;

#[derive(Data, Clone)]
pub struct GraphicsData {
	pub objects: OrdSet<RenderObject>,
	pub preview: Option<RenderObject>,
	pub tool: Rc<Box<dyn Tool>>,
}
