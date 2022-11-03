use std::sync::{Arc, Mutex};

use draw_tools::fractal_line_tool::FractalLineTool;
use draw_tools::selection_tool::SelectionTool;
use druid::im::ordset;
use druid::widget::{Button, Flex};
use druid::{AppLauncher, PlatformError, Widget, WindowDesc};

mod draw_tools;
mod render_objects;
mod widgets;
use widgets::{graphics_data::GraphicsData, graphics_scene_widget::*};

fn build_ui() -> impl Widget<GraphicsData> {
	let mut row = Flex::row();
	row.add_child(
		Flex::column()
			.with_child(Button::new("Fractal Line Tool").on_click(
				|_ctx, data: &mut GraphicsData, _env| {
					data.tool.lock().unwrap().disable(&mut data.objects);
					data.tool = Arc::new(Mutex::new(FractalLineTool::new()));
					data.tool.lock().unwrap().enable(&mut data.objects);
				},
			))
			.with_child(Button::new("Selection Tool").on_click(
				|_ctx, data: &mut GraphicsData, _env| {
					data.tool.lock().unwrap().disable(&mut data.objects);
					data.tool = Arc::new(Mutex::new(SelectionTool::new()));
					data.tool.lock().unwrap().enable(&mut data.objects);
				},
			)),
	);
	row.add_flex_child(GraphicsWidget::new(), 1.0);
	row
}

fn main() -> Result<(), PlatformError> {
	AppLauncher::with_window(WindowDesc::new(build_ui)).launch(GraphicsData {
		objects: ordset![],
		preview: None,
		tool: Arc::new(Mutex::new(FractalLineTool::new())),
	})?;
	Ok(())
}
