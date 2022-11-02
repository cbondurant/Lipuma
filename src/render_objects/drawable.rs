use druid::{kurbo::BezPath, Rect};

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
