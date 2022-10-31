use draw_tools::fractal_line_tool::FractalLineTool;
use druid::im::ordset;
use druid::widget::Flex;
use druid::{theme, AppLauncher, Color, PlatformError, Widget, WidgetExt, WindowDesc};
use std::rc::Rc;

mod draw_tools;
mod render_objects;
mod widgets;
use widgets::{graphics_data::GraphicsData, graphics_scene_widget::*};

fn build_ui() -> impl Widget<GraphicsData> {
	let mut row = Flex::row();
	row.add_flex_child(GraphicsWidget::new(), 1.0);
	row.debug_paint_layout()
}

fn main() -> Result<(), PlatformError> {
	AppLauncher::with_window(WindowDesc::new(build_ui))
		.configure_env(|env, _| {
			env.set(
				theme::WINDOW_BACKGROUND_COLOR,
				Color::rgba(0.0, 0.0, 0.0, 0.0),
			)
		})
		.launch(GraphicsData {
			objects: ordset![],
			preview: None,
			tool: Rc::new(Box::new(FractalLineTool::new())),
		})?;
	Ok(())
}
