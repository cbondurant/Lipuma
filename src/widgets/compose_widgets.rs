/// This document holds all of the functions that build composed widgets
///
/// Composed widgets are any that are built using the existing WidgetExt functions
/// and any Control wrappers. Im putting them here since they dont have a regular
/// struct organizational system to use instead.
use druid::{text::format::ParseFormatter, widget::*, Data, Lens, LensExt, Widget, WidgetExt};
use tool::ToolObj;

use crate::draw_tools::{tool, FractalLineTool, SelectionTool};

use super::graphics_data::GraphicsData;

pub fn settings_menu() -> impl Widget<GraphicsData> {
	tool::ToolObj::matcher()
		.fractal_line_tool(FractalLineTool::get_configuration())
		.selection_tool(SelectionTool::get_configuration())
		.lens(GraphicsData::tool)
}

pub fn tool_selection_button(tool: ToolObj, name: &str) -> impl Widget<GraphicsData> {
	Button::new(name).on_click(move |ctx, data: &mut GraphicsData, _env| {
		data.tool.disable(&mut data.objects);
		data.tool = tool;
		data.tool.enable(&mut data.objects);
		ctx.request_layout();
	})
}

pub fn integer_stepper<T: Data, L: Lens<T, i32> + 'static>(
	min: i32,
	max: i32,
	l: L,
) -> impl Widget<T> {
	Flex::row()
		.with_child(TextBox::new().with_formatter(ParseFormatter::new()))
		.with_child(Stepper::new().with_range(min as f64, max as f64))
		.lens(l.map(|val| *val as f64, |val, new| *val = new as i32))
}

pub fn slider_with_label<T: Data, L: Lens<T, f64> + 'static>(
	min: f64,
	max: f64,
	l: L,
) -> impl Widget<T> {
	Flex::row()
		.with_child(
			TextBox::new().with_formatter(ParseFormatter::with_format_fn(|v| format!("{:.2}", v))),
		)
		.with_child(Slider::new().with_range(min as f64, max as f64))
		.lens(l)
}
