use druid::{im::OrdSet, Data, Point};
use noise::OpenSimplex;
use rand::random;
use std::rc::Rc;

use crate::render_objects::{fractal_line::FractalLine, RenderObject};

use super::tool::Tool;

#[derive(Data, Clone, PartialEq, Eq, Debug)]
enum ToolState {
	Drawing,
	Standby,
}

#[derive(Data, Debug, Clone)]
pub struct FractalLineTool {
	preview: FractalLine,
	state: ToolState,
}

impl FractalLineTool {
	pub fn new() -> Self {
		Self {
			preview: FractalLine {
				start: Point::ZERO,
				end: Point::ZERO,
				noise: Rc::new(OpenSimplex::new(0)),
				width: 10.0,
				density: 0.05,
				samples: 1000,
			},
			state: ToolState::Standby,
		}
	}
	fn on_mouse_move(
		&mut self,
		event: &druid::MouseEvent,
		ctx: &mut druid::EventCtx,
		data: OrdSet<RenderObject>,
	) -> OrdSet<RenderObject> {
		match self.state {
			ToolState::Drawing => {
				ctx.set_handled();
				self.preview.end = event.pos;
			}
			ToolState::Standby => (),
		}
		data
	}

	fn on_mouse_down(
		&mut self,
		event: &druid::MouseEvent,
		ctx: &mut druid::EventCtx,
		data: OrdSet<RenderObject>,
	) -> OrdSet<RenderObject> {
		self.state = ToolState::Drawing;
		self.preview = FractalLine {
			start: event.pos,
			end: event.pos,
			noise: Rc::new(OpenSimplex::new(random())),
			width: 10.0,
			density: 0.05,
			samples: 1000,
		};
		ctx.set_handled();
		data
	}

	fn on_mouse_up(
		&mut self,
		event: &druid::MouseEvent,
		ctx: &mut druid::EventCtx,

		mut data: OrdSet<RenderObject>,
	) -> OrdSet<RenderObject> {
		match self.state {
			ToolState::Drawing => {
				self.preview.end = event.pos;
				let mut obj = self.get_preview().unwrap();
				obj.z = match data.get_max() {
					Some(obj) => obj.z + 1,
					None => 0,
				};
				self.state = ToolState::Standby;
				data.insert(obj);
				ctx.is_handled();
			}
			ToolState::Standby => (),
		}
		data
	}
}

impl Tool for FractalLineTool {
	fn enable(&mut self, _data: &mut OrdSet<RenderObject>) {
		self.state = ToolState::Standby;
	}

	fn disable(&mut self, data: &mut OrdSet<RenderObject>) {
		match self.state {
			ToolState::Drawing => {
				// get_preview always returns some when drawing
				data.insert(self.get_preview().unwrap());
			}
			ToolState::Standby => (),
		}
	}

	fn event(
		&mut self,
		event: &druid::Event,
		ctx: &mut druid::EventCtx,
		data: OrdSet<RenderObject>,
	) -> OrdSet<RenderObject> {
		match event {
			druid::Event::MouseDown(event) => self.on_mouse_down(event, ctx, data),
			druid::Event::MouseUp(event) => self.on_mouse_up(event, ctx, data),
			druid::Event::MouseMove(event) => self.on_mouse_move(event, ctx, data),
			_ => data,
		}
	}

	fn get_preview(&self) -> Option<RenderObject> {
		match self.state {
			ToolState::Drawing => Some(RenderObject::new(
				u32::MAX,
				Rc::new(Box::new(self.preview.clone())),
			)),
			ToolState::Standby => None,
		}
	}
}
