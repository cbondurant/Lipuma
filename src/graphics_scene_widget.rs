use std::rc::Rc;

use druid::im::OrdSet;
use druid::im::Vector;
use druid::Color;
use druid::Point;
use druid::RenderContext;
use druid::{Data, Widget};

use noise::OpenSimplex;

use crate::draw_tools::fractal_line_tool::FractalLineTool;
use crate::draw_tools::tool::Tool;
use crate::renderobject::RenderObject;

#[derive(Data, Clone, Debug)]
pub struct Line(Point, Point, Rc<OpenSimplex>);

#[derive(Data, Clone)]
pub struct GraphicsData {
	pub objects: OrdSet<RenderObject>,
	pub preview: Option<RenderObject>,
}

pub struct GraphicsWidget {
	change_list: OrdSet<RenderObject>,
	remove_list: Vector<RenderObject>,
	current_tool: Rc<Box<dyn Tool>>,
}

impl GraphicsWidget {
	pub fn new() -> Self {
		Self {
			change_list: OrdSet::new(),
			remove_list: Vector::new(),
			current_tool: Rc::new(Box::new(FractalLineTool::new())),
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
		Rc::get_mut(&mut self.current_tool)
			.unwrap()
			.event(event, ctx, data);
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
		data.preview = self.current_tool.get_preview();
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
					self.remove_list.push_front(old.clone());
					ctx.request_paint_rect(new.get_drawable().AABB());
					ctx.request_paint_rect(old.get_drawable().AABB());
				}
				druid::im::ordset::DiffItem::Remove(item) => {
					self.remove_list.push_front(item.clone());
					ctx.request_paint_rect(item.get_drawable().AABB());
				}
			}
		}

		if let (Some(old), Some(new)) = (&old_data.preview, &data.preview) {
			if !old.same(new) {
				self.change_list.insert(new.clone());
			}

			ctx.request_paint_rect(old.get_drawable().AABB());
			ctx.request_paint_rect(new.get_drawable().AABB());
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
		for object in &data.objects {
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

		//println!("{}, {}", self.change_list.len(), redraw_needed.len());
		ctx.save().unwrap();
		for robj in &self.change_list {
			ctx.fill(robj.drawable.AABB(), &Color::WHITE);
		}
		for robj in &redraw_needed {
			ctx.fill(robj.drawable.AABB(), &Color::WHITE);
		}
		// TODO: Fix draw order errors
		for robj in self.change_list.iter().chain(redraw_needed) {
			robj.paint(ctx, env);
		}
		if let Some(line) = &data.preview {
			line.paint(ctx, env);
		}
		ctx.restore().unwrap();
		self.change_list.clear();
	}
}
