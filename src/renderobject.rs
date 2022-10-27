use crate::drawable::Drawable;
use druid::Affine;
use druid::Color;
use druid::Data;
use druid::RenderContext;
use std::rc::Rc;

#[derive(Data, Clone)]
pub struct RenderObject {
	pub z: u32,
	pub transform: Affine,
	pub drawable: Rc<Box<dyn Drawable>>,
}

impl Ord for RenderObject {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.z.cmp(&other.z)
	}
}
impl PartialOrd for RenderObject {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.z.partial_cmp(&other.z)
	}
}

impl PartialEq for RenderObject {
	fn eq(&self, other: &Self) -> bool {
		self.z == other.z
	}
}

impl Eq for RenderObject {}

impl RenderObject {
	pub fn paint(&self, ctx: &mut druid::PaintCtx, env: &druid::Env) {
		ctx.with_save(|newctx| {
			newctx.transform(self.transform);
			//newctx.clip(self.drawable.AABB());
			//newctx.fill(self.drawable.AABB(), &Color::WHITE);
			self.drawable.paint(newctx, env, &self.transform);
		});
	}

	pub fn new(z: u32, drawable: Rc<Box<dyn Drawable>>) -> Self {
		Self {
			z,
			transform: Affine::new([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
			drawable,
		}
	}

	pub fn get_drawable(&self) -> Rc<Box<dyn Drawable>> {
		Rc::clone(&self.drawable)
	}

	#[allow(dead_code)] // Exists for possible debug use
	pub fn paint_bounds(&self, ctx: &mut druid::PaintCtx, _env: &druid::Env) {
		ctx.transform(self.transform);
		ctx.with_save(|new_ctx| new_ctx.stroke(self.drawable.AABB(), &Color::RED, 1.0))
	}

	pub fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, env: &druid::Env) {
		Rc::get_mut(&mut self.drawable)
			.unwrap()
			.event(ctx, event, env, &mut self.transform)
	}

	pub fn intersects(&self, rhs: &Self) -> bool {
		!self
			.drawable
			.AABB()
			.intersect(rhs.drawable.AABB())
			.is_empty()
	}
}
