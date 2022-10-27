use druid::im::ordset;
use druid::{theme, AppLauncher, Color, PlatformError, Widget, WindowDesc};

mod bound;
mod draw_tools;
mod drawable;
mod fractal_line;
mod graphics_scene_widget;
mod renderobject;
use graphics_scene_widget::*;

fn build_ui() -> impl Widget<GraphicsData> {
	GraphicsWidget::new()
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
		})?;
	Ok(())
}
