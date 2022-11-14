use druid::{im::OrdSet, Data, Point};
use rand::random;
use std::default::Default;

use crate::render_objects::{fractal_line::FractalNoise, Drawable, FractalLine, RenderObject};

use super::tool::Tool;

#[derive(Data, Clone, Copy, PartialEq, Eq, Debug)]
enum ToolState {
	Drawing,
	Standby,
}

#[derive(Data, Debug, Clone, Copy, PartialEq)]
pub struct FractalLineTool {
	preview: FractalLine,
	state: ToolState,
}

impl FractalLineTool {
	pub fn new() -> Self {
		Self {
			// We have to have a preview but this one will never be used
			preview: FractalLine {
				start: Point::ZERO,
				end: Point::ZERO,
				noise: FractalNoise::new(random(), 0.0, 0),
				..Default::default()
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
			noise: FractalNoise::new(random(), 0.35, 3),
			width: 5.0,
			density: 0.05,
			samples: 500,
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
				Drawable::FractalLine(self.preview),
			)),
			ToolState::Standby => None,
		}
	}
}

impl Default for FractalLineTool {
	fn default() -> Self {
		Self::new()
	}
}
