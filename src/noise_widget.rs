use std::rc::Rc;

use druid::im::OrdSet;
use druid::Affine;
use druid::Color;
use druid::Point;
use druid::RenderContext;
use druid::{Data, Widget};

use noise::OpenSimplex;

use crate::drawable::Drawable;
use crate::fractal_line::FractalLine;
use crate::renderobject::RenderObject;

#[derive(Data, Clone, Debug)]
pub struct Line(Point, Point, Rc<OpenSimplex>);

#[derive(Data, Clone)]
pub struct GraphicsData {
	pub objects: OrdSet<RenderObject>,
	pub preview: Option<FractalLine>,
}

pub enum GraphicsEngineState {
	Default,
	Drawing,
}

pub struct GraphicsWidget {
	pub state: GraphicsEngineState,
	change_list: OrdSet<RenderObject>,
	background_dirty: bool,
}

impl GraphicsWidget {
	pub fn new() -> Self {
		Self {
			state: GraphicsEngineState::Default,
			change_list: OrdSet::new(),
			background_dirty: true,
		}
	}

	fn enter_state(&mut self, new_state: GraphicsEngineState) {
		self.exit_state();
		self.state = new_state;
	}

	fn exit_state(&self) {
		match self.state {
			GraphicsEngineState::Default => (),
			GraphicsEngineState::Drawing => (),
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
		match event {
			druid::Event::WindowConnected => {}
			druid::Event::MouseDown(event) => match self.state {
				GraphicsEngineState::Default => {
					self.enter_state(GraphicsEngineState::Drawing);
					data.preview = Some(FractalLine {
						start: event.pos,
						end: event.pos,
						noise: Rc::new(OpenSimplex::new(rand::random())),
						width: 10.0,
						density: 0.05,
						samples: 1000,
					});
				}
				GraphicsEngineState::Drawing => (),
			},
			druid::Event::MouseUp(_) => match self.state {
				GraphicsEngineState::Default => (),
				GraphicsEngineState::Drawing => {
					data.objects.insert(RenderObject {
						transform: Affine::scale(1.0),
						drawable: Rc::new(Box::new(data.preview.take().unwrap())),
						z: match data.objects.get_max() {
							Some(v) => v.z + 1,
							None => 0,
						},
					});

					data.preview = None;
					self.enter_state(GraphicsEngineState::Default);
					ctx.request_paint();
				}
			},
			druid::Event::MouseMove(event) => {
				if let GraphicsEngineState::Drawing = self.state {
					if let Some(preview) = &mut data.preview {
						preview.end = event.pos;
					}
					ctx.request_paint();
				}
			}
			_ => (),
		}
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
		_ctx: &mut druid::UpdateCtx,
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
		for object in &self.change_list {
			for colltest in data.objects.iter() {
				if object.intersects(colltest) {
					redraw_needed.insert(colltest);
				}
			}
		}

		if self.background_dirty {
			ctx.clear(Color::WHITE);
			self.background_dirty = false;
		}

		//println!("{}, {}", self.change_list.len(), redraw_needed.len());
		ctx.save().unwrap();
		// This assumed redraw_needed only ever has one element, which I believe to be the case
		if let Some(ro) = redraw_needed.get_max() {
			ctx.clip(ro.drawable.AABB())
		}
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
			line.paint(ctx, env, &Affine::rotate(0.0));
		}
		ctx.restore().unwrap();
		self.change_list.clear();
	}
}
