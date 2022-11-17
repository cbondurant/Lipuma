use crate::draw_tools::{FractalLineTool, Tool};
use crate::render_objects::RenderObject;
use druid::im::OrdSet;
use druid::{Affine, Data, Lens, Vec2};

#[derive(Data, Clone, Lens)]
pub struct GraphicsData {
	pub objects: OrdSet<RenderObject>,
	pub preview: Option<RenderObject>,
	pub tool: Tool,
	pub transform: Affine,
}

impl GraphicsData {
	pub fn new() -> Self {
		Self {
			objects: OrdSet::new(),
			preview: None,
			tool: Tool::FractalLineTool(FractalLineTool::new()),
			transform: Affine::default(),
		}
	}

	// Get the transform that converts from canvas space to widget space
	pub fn get_trans_to_widget(&self) -> Affine {
		self.transform
	}

	// A transformation that includes only the translation of the matrix
	pub fn get_translation(&self) -> Affine {
		let mut coeff = self.transform.as_coeffs();
		coeff[0] = 1.0;
		coeff[1] = 0.0;
		coeff[2] = 1.0;
		coeff[3] = 0.0;
		Affine::new(coeff)
	}

	// A transformation that only includes the rotation and scale of the transform
	pub fn get_rot_scale(&self) -> Affine {
		let mut coeff = self.transform.as_coeffs();
		coeff[4] = 0.0;
		coeff[5] = 0.0;
		Affine::new(coeff)
	}

	// Scales the transform relative to a point p in canvas space.
	pub fn scale_around_point(&mut self, p: Vec2, s: f64) {
		self.transform *= Affine::translate(p) * Affine::scale(s) * Affine::translate(p).inverse();
	}

	// Rotates the transform around a point p in canvas space.
	pub fn rotate_around_point(&mut self, p: Vec2, th: f64) {
		self.transform *=
			Affine::translate(p) * Affine::rotate(th) * Affine::translate(p).inverse();
	}
}

impl Default for GraphicsData {
	fn default() -> Self {
		Self::new()
	}
}
