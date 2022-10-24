use crate::renderobject::RenderObject;
use druid::{im::Vector, Affine, Data, RenderContext};

#[derive(Data, Clone)]
pub struct RenderScene {
	transform: Affine,
	objects: Vector<RenderObject>,
}

impl RenderScene {
	fn paint(&mut self, ctx: &mut druid::PaintCtx, env: &druid::Env) {
		ctx.save().unwrap(); // The docs do not explain any reason why this would fail...
		ctx.transform(self.transform);
		for object in self.objects.iter_mut() {
			object.paint(ctx, env)
		}
		ctx.restore().unwrap(); // Ditto here.
	}
}
