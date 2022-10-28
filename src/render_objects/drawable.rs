use druid::{Affine, Rect};

pub trait Drawable {
	#[allow(non_snake_case)]
	fn AABB(&self) -> Rect;

	fn event(
		&mut self,
		ctx: &mut druid::EventCtx,
		event: &druid::Event,
		env: &druid::Env,
		sctx: &mut Affine,
	);
	fn paint(&self, ctx: &mut druid::PaintCtx, env: &druid::Env, sctx: &Affine);
}
