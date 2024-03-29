use super::drawable::DrawableObj;
use druid::{Affine, Color, Data, RenderContext};
use std::fmt::Debug;

#[derive(Data, Clone, Copy)]
pub struct RenderObject {
	pub transform: Affine,
	pub selected: bool,
	pub drawable: DrawableObj,
}

impl Debug for RenderObject {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("RenderObject")
			.field("transform", &self.transform)
			.field("drawable", &self.drawable.AABB())
			.finish()
	}
}

impl RenderObject {
	pub fn paint(&self, ctx: &mut druid::PaintCtx, env: &druid::Env) {
		ctx.with_save(|newctx| {
			newctx.transform(self.transform);
			self.drawable.paint(newctx, env, self);
		});
	}

	pub fn new(drawable: DrawableObj) -> Self {
		Self {
			transform: Affine::new([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
			drawable,
			selected: false,
		}
	}

	pub fn get_drawable(&self) -> &DrawableObj {
		&self.drawable
	}

	#[allow(dead_code)] // Exists for possible debug use
	pub fn paint_bounds(&self, ctx: &mut druid::PaintCtx, _env: &druid::Env) {
		ctx.transform(self.transform);
		ctx.with_save(|new_ctx| new_ctx.stroke(self.drawable.AABB(), &Color::RED, 1.0))
	}

	pub fn select(&mut self) {
		self.selected = true;
	}

	pub fn deselect(&mut self) {
		self.selected = false;
	}

	pub fn is_selected(&self) -> bool {
		self.selected
	}
}
