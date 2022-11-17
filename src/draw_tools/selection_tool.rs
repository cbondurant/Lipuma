use super::tool::Tool;
use druid::widget::{Flex, Label};
use druid::{im::OrdSet, Data, Event, Point, Rect};
use druid::{Lens, Widget};

use crate::render_objects::drawable::DrawableObj;
use crate::render_objects::{selection_rect::SelectionRect, RenderObject};

#[derive(Data, Debug, Clone, Copy, PartialEq, Eq)]
enum SelectionState {
	Selecting,
	Standby,
}

#[derive(Data, Debug, Clone, Copy, PartialEq, Lens)]
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
			state: SelectionState::Standby,
		}
	}

	fn update_selected(&self, data: &mut OrdSet<RenderObject>) {
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
								if !item.is_selected() {
									let mut new_item = *item;
									new_item.select();
									data.remove(item);
									data.insert(new_item);
								}
								continue 'outer;
							}
						}
						druid::kurbo::PathEl::ClosePath => todo!(),
					}
				}
			}

			if item.is_selected() {
				let mut new_item = *item;
				new_item.deselect();
				data.remove(item);
				data.insert(new_item);
			}
		}
	}

	pub fn get_configuration() -> impl Widget<Self> {
		Flex::column().with_child(Label::new("Selection Tool"))
	}
}

impl Tool for SelectionTool {
	fn enable(&mut self, _data: &mut OrdSet<RenderObject>) {}

	fn disable(&mut self, _data: &mut OrdSet<RenderObject>) {}

	fn event(
		&mut self,
		event: &druid::Event,
		_ctx: &mut druid::EventCtx,
		data: &mut OrdSet<RenderObject>,
	) {
		match event {
			Event::MouseDown(e) => {
				self.state = SelectionState::Selecting;
				self.start_coord = e.pos;
				self.end_coord = e.pos;
			}
			Event::MouseUp(_) => self.state = SelectionState::Standby,
			Event::MouseMove(e) => {
				if let SelectionState::Selecting = self.state {
					self.end_coord = e.pos;
					self.update_selected(data);
				}
			}
			_ => (),
		}
	}

	fn get_preview(&self) -> Option<RenderObject> {
		match self.state {
			SelectionState::Selecting => Some(RenderObject::new(
				u32::MAX,
				DrawableObj::SelectionRect(SelectionRect::new(Rect::from_points(
					self.start_coord,
					self.end_coord,
				))),
			)),
			SelectionState::Standby => None,
		}
	}
}

impl Default for SelectionTool {
	fn default() -> Self {
		Self::new()
	}
}
