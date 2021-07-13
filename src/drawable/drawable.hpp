#ifndef DRAWABLE_DRAWABLE_HPP
#define DRAWABLE_DRAWABLE_HPP

#include <QGraphicsObject>
#include <QDataStream>


namespace Lipuma {

	enum DrawableSerializeTypes: qint8 {SerializeFractalLine, SerializeFractalCurve};
	class Drawable : public QGraphicsObject {
	public:
		enum { DrawableItemType = UserType + 1};
		virtual void write(QDataStream&);
		QRectF boundingRect() const override;
		void paint(QPainter *painter, const QStyleOptionGraphicsItem *option,QWidget *widget) override;
		int type() const override;
		static qint8 drawableType();
		virtual void setFrequency(qreal);
		virtual qreal getFrequency() const;
		virtual void setGain(qreal);
	};
}

#endif