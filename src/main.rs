use druid::im::vector;
use druid::im::vector::Vector;
use druid::Color;
use druid::Point;
use druid::Rect;
use druid::RenderContext;
use druid::{AppLauncher, Data, PlatformError, Widget, WindowDesc};

#[derive(Data, Clone, Debug)]
struct Line(Point, Point);

#[derive(Data, Clone)]
struct GraphicsData {
    pub lines: Vector<Line>,
    pub preview: Option<Line>,
}

enum GraphicsEngineState {
    Default,
    Drawing { draw_start: Point },
}

struct GraphicsWidget {
    state: GraphicsEngineState,
}

impl GraphicsWidget {
    fn enter_state(&mut self, new_state: GraphicsEngineState) {
        self.exit_state();
        self.state = new_state;
    }

    fn exit_state(&self) {
        match self.state {
            GraphicsEngineState::Default => (),
            GraphicsEngineState::Drawing { draw_start: _ } => (),
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
            druid::Event::MouseDown(event) => match self.state {
                GraphicsEngineState::Default => self.enter_state(GraphicsEngineState::Drawing {
                    draw_start: event.pos,
                }),
                GraphicsEngineState::Drawing { draw_start: _ } => return,
            },
            druid::Event::MouseUp(event) => match self.state {
                GraphicsEngineState::Default => return,
                GraphicsEngineState::Drawing { draw_start } => {
                    data.lines.push_front(Line(draw_start, event.pos));
                    data.preview = None;
                    self.enter_state(GraphicsEngineState::Default);
                    ctx.request_paint();
                }
            },
            druid::Event::MouseMove(event) => {
                if let GraphicsEngineState::Drawing { draw_start } = self.state {
                    data.preview = Some(Line(draw_start, event.pos));
                    ctx.request_paint();
                }
            }
            _ => return,
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
        _old_data: &GraphicsData,
        _data: &GraphicsData,
        _env: &druid::Env,
    ) {
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

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &GraphicsData, _env: &druid::Env) {
        let size = ctx.size();
        ctx.fill(Rect::from_origin_size(Point::ORIGIN, size), &Color::WHITE);
        for Line(start, end) in data.lines.iter() {
            let t = druid::piet::kurbo::Line::new((start.x, start.y), (end.x, end.y));
            ctx.stroke(t, &Color::BLACK, 1.0);
        }
        if let Some(Line(start, end)) = data.preview {
            ctx.stroke(
                druid::piet::kurbo::Line::new(start, end),
                &Color::BLACK,
                1.0,
            )
        }
    }
}

fn build_ui() -> impl Widget<GraphicsData> {
    GraphicsWidget {
        state: GraphicsEngineState::Default,
    }
}

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(build_ui)).launch(GraphicsData {
        lines: vector![],
        preview: None,
    })?;
    Ok(())
}
