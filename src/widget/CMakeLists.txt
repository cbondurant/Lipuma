add_library(widget
	toolSelector.cpp
	canvas.cpp
	propertiesMenu.cpp
)

target_link_libraries(widget Qt5::Gui Qt5::Widgets)