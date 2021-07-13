#include <math.h>

#include <QSlider>
#include <QFormLayout>

#include "widget/propertiesMenu.hpp"
#include "drawable/drawable.hpp"

namespace Lipuma{
	PropertiesMenu::PropertiesMenu(Canvas *canvas, QWidget *w): QWidget(w), canvas(canvas){
		layout = new QFormLayout(this);
		setLayout(layout);
		QSlider *frequencyslider = new QSlider(Qt::Horizontal);
		layout->addRow("Frequency", frequencyslider);
		frequencyslider->setMinimum(0);
		frequencyslider->setMaximum(100);
		frequencyslider->setTracking(true);
		connect(frequencyslider, &QSlider::valueChanged, this, &Lipuma::PropertiesMenu::frequencyChanged);

		QSlider *gainSlider = new QSlider(Qt::Horizontal);
		layout->addRow("Gain", gainSlider);
		gainSlider->setMinimum(0);
		gainSlider->setMaximum(100);
		gainSlider->setTracking(true);
		connect(gainSlider, &QSlider::valueChanged, this, &Lipuma::PropertiesMenu::gainChanged);

	}

	PropertiesMenu::~PropertiesMenu(){}

	void PropertiesMenu::frequencyChanged(int newfreq){
		float freq = pow(1.02,(float)newfreq/50)-1;
		for (auto i : items){
			dynamic_cast<Drawable*>(i)->setFrequency(freq);
		}
	}

	void PropertiesMenu::gainChanged(int newfreq){
		float freq = (float)newfreq/100;
		for (auto i : items){
			dynamic_cast<Drawable*>(i)->setGain(freq);
		}
	}


	void PropertiesMenu::selectionUpdated(){
		items = canvas->scene()->selectedItems();
	}

}