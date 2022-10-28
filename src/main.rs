use druid::im::ordset;
use druid::{theme, AppLauncher, Color, PlatformError, Widget, WindowDesc};

mod draw_tools;
mod render_objects;
mod widgets;
use widgets::graphics_scene_widget::*;

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
