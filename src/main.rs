use draw_tools::FractalLineTool;
use draw_tools::SelectionTool;
use draw_tools::Tool;
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
					data.tool.disable(&mut data.objects);
					data.tool = Tool::FractalLineTool(FractalLineTool::new());
					data.tool.enable(&mut data.objects);
				},
			))
			.with_child(Button::new("Selection Tool").on_click(
				|_ctx, data: &mut GraphicsData, _env| {
					data.tool.disable(&mut data.objects);
					data.tool = Tool::SelectionTool(SelectionTool::new());
					data.tool.enable(&mut data.objects);
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
		tool: Tool::FractalLineTool(FractalLineTool::new()),
	})?;
	Ok(())
}
