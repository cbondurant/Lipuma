use std::sync::{Arc, Mutex};

use crate::draw_tools::tool::Tool;
use crate::render_objects::RenderObject;
use druid::im::OrdSet;
use druid::Data;

#[derive(Data, Clone)]
pub struct GraphicsData {
	pub objects: OrdSet<RenderObject>,
	pub preview: Option<RenderObject>,
	pub tool: Arc<Mutex<dyn Tool>>,
}
