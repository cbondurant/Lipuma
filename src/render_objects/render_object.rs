use super::drawable::Drawable;
use druid::Affine;
use druid::Color;
use druid::Data;
use druid::RenderContext;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Data, Clone)]
pub struct RenderObject {
	pub z: u32,
	pub transform: Affine,
	pub drawable: Rc<Box<dyn Drawable>>,
}

impl Debug for RenderObject {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("RenderObject")
			.field("z", &self.z)
			.field("transform", &self.transform)
			.field("drawable", &self.drawable.AABB())
			.finish()
	}
}

impl Ord for RenderObject {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		match self.z.cmp(&other.z) {
			std::cmp::Ordering::Equal => {
				Rc::as_ptr(&self.drawable).cmp(&Rc::as_ptr(&other.drawable))
			}
			ord => ord,
		}
	}
}
impl PartialOrd for RenderObject {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		match self.z.partial_cmp(&other.z) {
			Some(std::cmp::Ordering::Equal) => {
				Rc::as_ptr(&self.drawable).partial_cmp(&Rc::as_ptr(&other.drawable))
			}
			Some(ord) => Some(ord),
			None => None,
		}
	}
}

impl PartialEq for RenderObject {
	fn eq(&self, other: &Self) -> bool {
		self.z == other.z && Rc::ptr_eq(&self.drawable, &other.drawable)
	}
}

impl Eq for RenderObject {}

impl RenderObject {
	pub fn paint(&self, ctx: &mut druid::PaintCtx, env: &druid::Env) {
		ctx.with_save(|newctx| {
			newctx.transform(self.transform);
			//newctx.clip(self.drawable.AABB());
			//newctx.fill(self.drawable.AABB(), &Color::WHITE);
			self.drawable.paint(newctx, env, &self);
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
}
