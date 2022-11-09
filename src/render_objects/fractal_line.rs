use druid::{
	kurbo::{BezPath, PathEl, Shape},
	Color, Data, Point, Rect, RenderContext, Vec2,
};

use super::{drawable::Drawable, RenderObject};

#[derive(Data, Clone, Copy, Debug, Default, PartialEq)]
pub struct FractalNoise {
	seed: u32,
	pub laurancity: f64,
	pub octaves: i8,
}

impl FractalNoise {
	pub fn new(seed: u32, laurancity: f64, octaves: i8) -> Self {
		Self {
			seed,
			laurancity,
			octaves,
		}
	}

	// Information taken from skeeto/hash-prospector
	#[inline(always)]
	fn get_hash(mut i: u32) -> u32 {
		i ^= i >> 16;
		i = i.overflowing_mul(0x21f0aaad).0;
		i ^= i >> 15;
		i = i.overflowing_mul(0xd35a2d97).0;
		i ^= i >> 15;
		i
	}

	#[inline(always)]
	fn smooth_step(start: f64, end: f64, x: f64) -> f64 {
		start + (((3.0 * x.powi(2)) - (2.0 * x.powi(3))) * (end - start))
	}

	pub fn get(&self, distance: f64) -> f64 {
		let mut val = 0.0;
		for i in 1..self.octaves {
			let dist_scaled = distance * (2.0 as f64).powi(i.into()) as f64;
			val += FractalNoise::smooth_step(
				((Self::get_hash(self.seed.wrapping_mul(dist_scaled.floor() as u32)) % 3) as i32
					- 1) as f64,
				((Self::get_hash(self.seed.wrapping_mul(dist_scaled.ceil() as u32)) % 3) as i32 - 1)
					as f64,
				dist_scaled.fract(),
			) * self.laurancity.powi(i.into())
		}
		val
	}
}

#[derive(Data, Clone, Copy, Debug, Default, PartialEq)]
pub struct FractalLine {
	pub start: Point,
	pub end: Point,
	pub noise: FractalNoise,
	pub width: f64,
	pub density: f64,
	pub samples: i32,
}

impl FractalLinePathIter {
	fn smooth_to_zero(x: f64) -> f64 {
		1.0 - ((2.0 * x) - 1.0).powi(16)
	}
}

pub struct FractalLinePathIter {
	i: i32,
	segments: i32,
	line_data: FractalLine,
	real_length: f64,
	perpendicular: Vec2,
}

impl FractalLinePathIter {
	pub fn new(line_data: &FractalLine, segments: i32) -> Self {
		let dir = line_data.start - line_data.end;
		let real_length = dir.to_point().distance(Point::ZERO);
		let perpendicular = Vec2::new(dir.y, -dir.x).normalize();
		Self {
			i: 0,
			segments,
			line_data: line_data.clone(),
			real_length,
			perpendicular,
		}
	}
}

impl Iterator for FractalLinePathIter {
	type Item = PathEl;

	fn next(&mut self) -> Option<Self::Item> {
		if self.i > self.segments {
			return None;
		}
		let index = self.i as f64 / self.segments as f64;
		self.i += 1;

		let simplex_distance = self.real_length * index * self.line_data.density;
		let simplex = self.line_data.noise.get(simplex_distance) * 3.0;

		Some(druid::piet::kurbo::PathEl::LineTo(
			self.line_data.start.lerp(self.line_data.end, index)
				+ self.perpendicular * self.line_data.width * Self::smooth_to_zero(index) * simplex,
		))
	}
}

impl Shape for FractalLine {
	type PathElementsIter = FractalLinePathIter;

	fn path_elements(&self, _tolerance: f64) -> Self::PathElementsIter {
		FractalLinePathIter::new(self, self.samples)
	}

	fn area(&self) -> f64 {
		0.0
	}

	fn perimeter(&self, _accuracy: f64) -> f64 {
		todo!()
	}

	fn winding(&self, _: Point) -> i32 {
		0
	}

	fn bounding_box(&self) -> Rect {
		Rect::from_points(self.start, self.end).inflate(self.width * 10.5, self.width * 10.5)
	}
}

impl Drawable for FractalLine {
	fn AABB(&self) -> Rect {
		self.bounding_box()
	}

	fn fine_collision_shape(&self, tolerance: f64) -> BezPath {
		self.to_path(tolerance)
	}

	#[allow(unused_variables)]
	fn event(
		&mut self,
		ctx: &mut druid::EventCtx,
		event: &druid::Event,
		env: &druid::Env,
		sctx: &RenderObject,
	) {
		todo!()
	}

	fn paint(&self, ctx: &mut druid::PaintCtx, _env: &druid::Env, sctx: &RenderObject) {
		ctx.stroke(
			self,
			if sctx.is_selected() {
				&Color::RED
			} else {
				&Color::BLACK
			},
			1.0,
		);
	}
}
