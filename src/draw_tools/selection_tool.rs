use std::rc::Rc;

use super::tool::Tool;
use druid::{im::OrdSet, Data, Event, Point, Rect};

use crate::render_objects::{selection_rect::SelectionRect, RenderObject};

#[derive(Data, Debug, Clone)]
pub struct SelectionTool {
	start_coord: Point,
	end_coord: Point,
	is_active: bool,
}

impl SelectionTool {
	pub fn new() -> Self {
		Self {
			start_coord: Point::ZERO,
			end_coord: Point::ZERO,
			is_active: false,
		}
	}
}

impl Tool for SelectionTool {
	fn enable(&mut self, _data: &mut OrdSet<RenderObject>) {
		()
	}

	fn disable(&mut self, _data: &mut OrdSet<RenderObject>) {
		()
	}

	fn event(
		&mut self,
		event: &druid::Event,
		_ctx: &mut druid::EventCtx,
		data: OrdSet<RenderObject>,
	) -> OrdSet<RenderObject> {
		match event {
			Event::MouseDown(e) => {
				self.is_active = true;
				self.start_coord = e.pos;
			}
			Event::MouseUp(_) => self.is_active = false,
			Event::MouseMove(e) => self.end_coord = e.pos,
			_ => (),
		}
		data
	}

	fn get_preview(&self) -> Option<RenderObject> {
		if self.is_active {
			Some(RenderObject::new(
				u32::MAX,
				Rc::new(Box::new(SelectionRect::new(Rect::from_points(
					self.start_coord,
					self.end_coord,
				)))),
			))
		} else {
			None
		}
	}
}
