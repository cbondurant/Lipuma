#include "drawable/drawable.hpp"

#include <QGraphicsSceneMouseEvent>
#include <QPainter>

namespace Lipuma {

	void Drawable::write(QDataStream& is){
		return;
	}

	QRectF Drawable::boundingRect() const {return QRectF();}
	void Drawable::paint(QPainter*, const QStyleOptionGraphicsItem*,QWidget*){} ;
	int Drawable::type() const {return Type;}

	qreal Drawable::getFrequency() const{}

	void Drawable::setFrequency(qreal f){}

	void Drawable::setGain(qreal g){}
}