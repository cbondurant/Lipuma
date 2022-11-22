use super::tool::Tool;
use druid::im::Vector;
use druid::kurbo::Shape;
use druid::widget::{Flex, Label};
use druid::{Data, Event, Lens, Point, Rect, Widget};

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

	fn update_selected(&self, data: &mut Vector<RenderObject>) {
		let bound = Rect::from_points(self.start_coord, self.end_coord);
		'outer: for item in data.iter_mut() {
			if !bound.intersect(item.drawable.AABB()).is_empty() {
				for segment in item.drawable.fine_collision_shape(5.0).segments() {
					match segment {
						druid::kurbo::PathSeg::Line(l) => {
							for check in bound.path_segments(0.01) {
								if !check.intersect_line(l).is_empty() {
									if !item.is_selected() {
										item.select();
									}
									continue 'outer;
								}
							}
						}
						_ => todo!(),
					}
				}
			}

			if item.is_selected() {
				item.deselect();
			}
		}
	}

	pub fn get_configuration() -> impl Widget<Self> {
		Flex::column().with_child(Label::new("Selection Tool"))
	}
}

impl Tool for SelectionTool {
	fn enable(&mut self, _data: &mut Vector<RenderObject>) {}

	fn disable(&mut self, _data: &mut Vector<RenderObject>) {}

	fn event(
		&mut self,
		event: &druid::Event,
		_ctx: &mut druid::EventCtx,
		data: &mut Vector<RenderObject>,
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
			SelectionState::Selecting => Some(RenderObject::new(DrawableObj::SelectionRect(
				SelectionRect::new(Rect::from_points(self.start_coord, self.end_coord)),
			))),
			SelectionState::Standby => None,
		}
	}
}

impl Default for SelectionTool {
	fn default() -> Self {
		Self::new()
	}
}
