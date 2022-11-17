use druid::widget::Flex;
use druid::{AppLauncher, PlatformError, Widget, WindowDesc};
use rust_lipuma::draw_tools::{FractalLineTool, SelectionTool, Tool};

use rust_lipuma::widgets::compose_widgets::*;
use rust_lipuma::widgets::{graphics_data::GraphicsData, graphics_scene_widget::*};

fn build_ui() -> impl Widget<GraphicsData> {
	let mut row = Flex::row();
	row.add_child(
		Flex::column()
			.with_child(tool_selection_button(
				Tool::FractalLineTool(FractalLineTool::default()),
				"Fractal Line Tool",
			))
			.with_child(tool_selection_button(
				Tool::SelectionTool(SelectionTool::default()),
				"Selection Tool",
			)),
	);
	row.add_flex_child(GraphicsWidget::new(), 1.0);
	row.add_child(settings_menu());
	row
}

fn main() -> Result<(), PlatformError> {
	AppLauncher::with_window(WindowDesc::new(build_ui)).launch(GraphicsData::new())?;
	Ok(())
}
