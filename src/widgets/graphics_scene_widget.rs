use std::rc::Rc;

use druid::im::OrdSet;
use druid::Color;
use druid::Point;
use druid::RenderContext;
use druid::{Data, Widget};

use noise::OpenSimplex;

use super::graphics_data::GraphicsData;
use crate::render_objects::RenderObject;

#[derive(Data, Clone, Debug)]
pub struct Line(Point, Point, Rc<OpenSimplex>);

pub struct GraphicsWidget {
	change_list: OrdSet<RenderObject>,
}

impl GraphicsWidget {
	pub fn new() -> Self {
		Self {
			change_list: OrdSet::new(),
		}
	}
}

impl Widget<GraphicsData> for GraphicsWidget {
	fn event(
		&mut self,
		ctx: &mut druid::EventCtx,
		event: &druid::Event,
		data: &mut GraphicsData,
		_env: &druid::Env,
	) {
		let muttool = Rc::make_mut(&mut data.tool);
		data.objects = muttool.event(event, ctx, data.objects.clone());
		if !ctx.is_handled() {
			#[allow(clippy::single_match)]
			// We expect to match other expressions later, but this is the only one that matters now
			match event {
				druid::Event::WindowSize(_) => {
					// Need to request full repaint to ensure everything draws correctly
					ctx.request_paint();
				}
				_ => (),
			}
		}
		data.preview = muttool.get_preview();
		data.tool = Rc::new(muttool.clone());
	}

	fn lifecycle(
		&mut self,
		_ctx: &mut druid::LifeCycleCtx,
		_event: &druid::LifeCycle,
		_data: &GraphicsData,
		_env: &druid::Env,
	) {
	}

	fn update(
		&mut self,
		ctx: &mut druid::UpdateCtx,
		old_data: &GraphicsData,
		data: &GraphicsData,
		_env: &druid::Env,
	) {
		// TODO: This does not handle mutated objects, only newly added ones
		// I also think that due to how druid handles things, this will only
		// ever have one element...

		self.change_list = old_data
			.objects
			.diff(&data.objects)
			.filter_map(|diffitem| match diffitem {
				druid::im::ordset::DiffItem::Add(item) => Some(item.clone()),
				druid::im::ordset::DiffItem::Update { old: _, new } => Some(new.clone()),
				druid::im::ordset::DiffItem::Remove(_) => None,
			})
			.collect();

		for diff in old_data.objects.diff(&data.objects) {
			match diff {
				druid::im::ordset::DiffItem::Add(item) => {
					self.change_list.insert(item.clone());
					ctx.request_paint_rect(item.get_drawable().AABB());
				}
				druid::im::ordset::DiffItem::Update { old, new } => {
					self.change_list.insert(new.clone());
					ctx.request_paint_rect(new.get_drawable().AABB());
					ctx.request_paint_rect(old.get_drawable().AABB());
				}
				druid::im::ordset::DiffItem::Remove(item) => {
					ctx.request_paint_rect(item.get_drawable().AABB());
				}
			}
		}

		match (&old_data.preview, &data.preview) {
			(Some(old), Some(new)) => {
				if !old.same(new) {
					self.change_list.insert(new.clone());
				}

				ctx.request_paint_rect(old.get_drawable().AABB());
				ctx.request_paint_rect(new.get_drawable().AABB());
			}
			(Some(old), None) => {
				ctx.request_paint_rect(old.get_drawable().AABB());
			}
			(None, Some(new)) => {
				self.change_list.insert(new.clone());
				ctx.request_paint_rect(new.get_drawable().AABB());
			}
			(None, None) => (),
		}
	}

	fn layout(
		&mut self,
		_ctx: &mut druid::LayoutCtx,
		bc: &druid::BoxConstraints,
		_data: &GraphicsData,
		_env: &druid::Env,
	) -> druid::Size {
		bc.max()
	}

	fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &GraphicsData, env: &druid::Env) {
		let mut redraw_needed = OrdSet::new();
		for object in data.objects.iter().cloned() {
			if !object
				.get_drawable()
				.AABB()
				.intersect(ctx.region().bounding_box())
				.is_empty()
			{
				redraw_needed.insert(object);
			}
		}

		ctx.clear(Color::WHITE);

		ctx.save().unwrap();
		for robj in self.change_list.to_owned().union(redraw_needed) {
			robj.paint(ctx, env);
		}
		ctx.restore().unwrap();
		self.change_list.clear();
	}
}
