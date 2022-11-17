use druid::{im::OrdSet, widget::*, Data, Lens, LensExt, Point, Widget};
use rand::random;
use std::default::Default;

use crate::{
	render_objects::{fractal_line::FractalNoise, Drawable, FractalLine, RenderObject},
	widgets::compose_widgets::{integer_stepper, slider_with_label},
};

use super::tool::Tool;

#[derive(Data, Clone, Copy, PartialEq, Eq, Debug)]
enum ToolState {
	Drawing,
	Standby,
}

#[derive(Data, Debug, Clone, Copy, PartialEq, Lens)]
pub struct FractalLineTool {
	preview: FractalLine,
	state: ToolState,
	default_width: f64,
	wavelength: f64,
	default_sample_distance: f64,
	default_offset: f64,
	default_laurancity: f64,
	default_octaves: i8,
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
			default_width: 5.0,
			wavelength: 5.0,
			default_sample_distance: 2.0,
			default_offset: 5.0,
			default_octaves: 3,
			default_laurancity: 0.35,
		}
	}

	fn on_mouse_move(
		&mut self,
		event: &druid::MouseEvent,
		ctx: &mut druid::EventCtx,
		_data: &mut OrdSet<RenderObject>,
	) {
		match self.state {
			ToolState::Drawing => {
				ctx.set_handled();
				self.preview.end = event.pos;
			}
			ToolState::Standby => (),
		}
	}

	fn on_mouse_down(
		&mut self,
		event: &druid::MouseEvent,
		ctx: &mut druid::EventCtx,
		_data: &mut OrdSet<RenderObject>,
	) {
		self.state = ToolState::Drawing;
		self.preview = FractalLine {
			start: event.pos,
			end: event.pos,
			noise: FractalNoise::new(random(), self.default_laurancity, self.default_octaves),
			width: self.default_width,
			wavelength: self.wavelength,
			sample_distance: self.default_sample_distance,
			offset: self.default_offset,
		};
		ctx.set_handled();
	}

	fn on_mouse_up(
		&mut self,
		event: &druid::MouseEvent,
		ctx: &mut druid::EventCtx,

		data: &mut OrdSet<RenderObject>,
	) {
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
	}

	pub fn get_configuration() -> impl Widget<Self> {
		Flex::column()
			.with_child(Label::new("Fractal Line Tool"))
			.with_child(slider_with_label(0.0, 10.0, Self::default_width))
			.with_child(slider_with_label(1.0, 10.0, Self::wavelength))
			.with_child(slider_with_label(0.1, 10.0, Self::default_sample_distance))
			.with_child(integer_stepper(
				0,
				5,
				Self::default_octaves.map(|v| *v as i32, |v, new| *v = new as i8),
			))
			.with_child(slider_with_label(0.0, 10.0, Self::default_offset))
			.with_child(slider_with_label(0.0, 0.5, Self::default_laurancity))
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
		data: &mut OrdSet<RenderObject>,
	) {
		match event {
			druid::Event::MouseDown(event) => self.on_mouse_down(event, ctx, data),
			druid::Event::MouseUp(event) => self.on_mouse_up(event, ctx, data),
			druid::Event::MouseMove(event) => self.on_mouse_move(event, ctx, data),
			_ => (),
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
