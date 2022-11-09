use crate::draw_tools::Tool;
use crate::render_objects::RenderObject;
use druid::im::OrdSet;
use druid::Data;

#[derive(Data, Clone)]
pub struct GraphicsData {
	pub objects: OrdSet<RenderObject>,
	pub preview: Option<RenderObject>,
	pub tool: Tool,
}
