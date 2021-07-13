#ifndef WIDGET_PROPERTIESMENU_HPP
#define WIDGET_PROPERTIESMENU_HPP

#include <QWidget>
#include <QGraphicsScene>
#include <QFormLayout>

#include "widget/canvas.hpp"

namespace Lipuma{
	class PropertiesMenu : public QWidget{
	Q_OBJECT
	public:
		PropertiesMenu(Canvas* canvas, QWidget *w = nullptr);
		~PropertiesMenu();
		void selectionUpdated();

	public slots:
		void frequencyChanged(int);
		void gainChanged(int);

	private:
		QFormLayout *layout;
		Canvas *canvas;
		QList<QGraphicsItem*> items;
	};
}

#endif