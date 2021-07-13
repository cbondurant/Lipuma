#include <math.h>

#include <QSlider>
#include <QFormLayout>

#include "widget/propertiesMenu.hpp"
#include "drawable/drawable.hpp"

namespace Lipuma{
	PropertiesMenu::PropertiesMenu(Canvas *canvas, QWidget *w): QWidget(w), canvas(canvas){
		layout = new QFormLayout(this);
		setLayout(layout);
		QSlider *slider = new QSlider(Qt::Horizontal);
		layout->addRow("Frequency", slider);
		slider->setMinimum(0);
		slider->setMaximum(100);
		slider->setTracking(true);
		connect(slider, &QSlider::valueChanged, this, &Lipuma::PropertiesMenu::frequencyChanged);
	}

	PropertiesMenu::~PropertiesMenu(){}

	void PropertiesMenu::frequencyChanged(int newfreq){
		float freq = pow(1.02,(float)newfreq/50)-1;
		for (auto i : items){
			dynamic_cast<Drawable*>(i)->setFrequency(freq);
		}
	}

	void PropertiesMenu::selectionUpdated(){
		items = canvas->scene()->selectedItems();
	}

}