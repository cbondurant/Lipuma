use super::fractal_line::FractalLine;
use super::selection_rect::SelectionRect;
use druid::{kurbo::BezPath, Data, Rect};
use trait_enum::trait_enum;

use super::RenderObject;

pub trait Drawable {
	#[allow(non_snake_case)]
	fn AABB(&self) -> Rect;
	fn fine_collision_shape(&self, tolerance: f64) -> BezPath;

	fn event(
		&mut self,
		ctx: &mut druid::EventCtx,
		event: &druid::Event,
		env: &druid::Env,
		sctx: &RenderObject,
	);
	fn paint(&self, ctx: &mut druid::PaintCtx, env: &druid::Env, sctx: &RenderObject);
}

trait_enum! {
	#[derive(Data, Clone, Copy)]
	pub enum DrawableObj: Drawable {
		FractalLine,
		SelectionRect
	}
}
