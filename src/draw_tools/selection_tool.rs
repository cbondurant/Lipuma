use super::tool::Tool;
use druid::{im::OrdSet, Data, Event, Point, Rect};

use crate::render_objects::drawable::DrawableObj;
use crate::render_objects::{selection_rect::SelectionRect, RenderObject};

#[derive(Data, Debug, Clone, Copy, PartialEq, Eq)]
enum SelectionState {
	Active,
	Inactive,
}

#[derive(Data, Debug, Clone, Copy)]
pub struct SelectionTool {
	start_coord: Point,
	end_coord: Point,
	state: SelectionState,
}

impl SelectionTool {
	pub fn new() -> Self {
		Self {
			start_coord: Point::ZERO,
			end_coord: Point::ZERO,
			state: SelectionState::Inactive,
		}
	}

	fn update_selected(&self, mut data: OrdSet<RenderObject>) -> OrdSet<RenderObject> {
		let bound = Rect::from_points(self.start_coord, self.end_coord);
		'outer: for item in &data.clone() {
			if !bound.intersect(item.drawable.AABB()).is_empty() {
				for segment in item.drawable.fine_collision_shape(1.0) {
					match segment {
						druid::kurbo::PathEl::MoveTo(p)
						| druid::kurbo::PathEl::LineTo(p)
						| druid::kurbo::PathEl::QuadTo(_, p)
						| druid::kurbo::PathEl::CurveTo(_, _, p) => {
							if bound.contains(p) {
								let mut new_item = *item;
								new_item.select();
								data.remove(item);
								data.insert(new_item);
								continue 'outer;
							}
						}
						druid::kurbo::PathEl::ClosePath => todo!(),
					}
				}
			}

			let mut new_item = *item;
			new_item.deselect();
			data.remove(item);
			data.insert(new_item);
		}
		data
	}
}

impl Tool for SelectionTool {
	fn enable(&mut self, _data: &mut OrdSet<RenderObject>) {}

	fn disable(&mut self, _data: &mut OrdSet<RenderObject>) {}

	fn event(
		&mut self,
		event: &druid::Event,
		_ctx: &mut druid::EventCtx,
		data: OrdSet<RenderObject>,
	) -> OrdSet<RenderObject> {
		match event {
			Event::MouseDown(e) => {
				self.state = SelectionState::Active;
				self.start_coord = e.pos;
				self.end_coord = e.pos;
			}
			Event::MouseUp(_) => self.state = SelectionState::Inactive,
			Event::MouseMove(e) => {
				if let SelectionState::Active = self.state {
					self.end_coord = e.pos;
					return self.update_selected(data);
				}
			}
			_ => (),
		}
		data
	}

	fn get_preview(&self) -> Option<RenderObject> {
		match self.state {
			SelectionState::Active => Some(RenderObject::new(
				u32::MAX,
				DrawableObj::SelectionRect(SelectionRect::new(Rect::from_points(
					self.start_coord,
					self.end_coord,
				))),
			)),
			SelectionState::Inactive => None,
		}
	}
}
