use druid::{
	piet::{PaintBrush, StrokeStyle},
	Color, Rect, RenderContext,
};

use super::drawable::Drawable;

const SELECTION_BRUSH: PaintBrush = PaintBrush::Color(Color::BLACK);

pub struct SelectionRect {
	rect: Rect,
}

impl SelectionRect {
	pub fn new(rect: Rect) -> Self {
		Self { rect }
	}
}

impl Drawable for SelectionRect {
	fn AABB(&self) -> Rect {
		self.rect.inflate(1.0, 1.0)
	}

	fn event(
		&mut self,
		_ctx: &mut druid::EventCtx,
		_event: &druid::Event,
		_env: &druid::Env,
		_sctx: &super::RenderObject,
	) {
		todo!()
	}

	fn paint(&self, ctx: &mut druid::PaintCtx, env: &druid::Env, sctx: &super::RenderObject) {
		ctx.stroke_styled(
			self.rect,
			&SELECTION_BRUSH,
			1.0,
			&StrokeStyle::new().dash(vec![3.0, 3.0], 0.0),
		);
	}
}
