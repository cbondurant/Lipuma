use druid::{
	kurbo::{PathEl, Shape},
	Affine, Color, Data, Point, Rect, RenderContext, Vec2,
};
use noise::{NoiseFn, OpenSimplex};
use std::rc::Rc;

use crate::drawable::Drawable;

#[derive(Data, Clone)]
pub struct FractalLine {
	pub start: Point,
	pub end: Point,
	pub noise: Rc<OpenSimplex>,
	pub width: f64,
	pub density: f64,
	pub samples: i32,
}

impl FractalLinePathIter {
	fn smooth_to_zero(x: f64) -> f64 {
		1.0 - (2.0 * (x - 0.5).powi(6))
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
		let simplex = self.line_data.noise.get([simplex_distance, 0.0]) * 3.0;
		Some(druid::piet::kurbo::PathEl::LineTo(
			self.line_data.start.lerp(self.line_data.end, index)
				+ self.line_data.width * self.perpendicular * simplex * Self::smooth_to_zero(index),
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
		Rect::from_points(self.start, self.end).inflate(self.width, self.width * 1.5)
	}
}

impl Drawable for FractalLine {
	fn AABB(&self) -> Rect {
		self.bounding_box()
	}

	#[allow(unused_variables)]
	fn event(
		&mut self,
		ctx: &mut druid::EventCtx,
		event: &druid::Event,
		env: &druid::Env,
		sctx: &mut Affine,
	) {
		todo!()
	}

	fn paint(&self, ctx: &mut druid::PaintCtx, _env: &druid::Env, _sctx: &Affine) {
		ctx.stroke(self, &Color::BLACK, 1.0);
	}
}
