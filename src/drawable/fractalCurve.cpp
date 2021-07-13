#include "fractalCurve.hpp"

#include <cmath>
#include <random>
#include <iostream>

#include <FastNoise/FastNoise.h>
#include <QColor>
#include <QGraphicsItem>
#include <QPainter>
#include <QRectF>
#include <QStyleOptionGraphicsItem>
#include <QWidget>

#include "math/bezier.hpp"
#include "drawable/editPoint.hpp"
#include "math/points.hpp"



namespace Lipuma {

	std::default_random_engine FractalCurve::rand;

	void FractalCurve::initalizeNoise(){
		seed = FractalCurve::rand();
		setFlag(QGraphicsItem::ItemIsSelectable);
		noise = FastNoise::New<FastNoise::FractalFBm>();
		noise->SetSource(FastNoise::New<FastNoise::Simplex>());
		setFrequency(0.02);
		noise->SetOctaveCount(5);
		noise->SetLacunarity(2.0f);
		noise->SetGain(.9);
	}

	void FractalCurve::initalizeEditPoints(){
		startPt = new EditPoint();
		startPt->setParentItem(this);
		startPt->setVisible(false);
		connect(startPt, &EditPoint::pointMoved, this, &FractalCurve::setStart);

		innerStartPt = new EditPoint();
		innerStartPt->setParentItem(this);
		innerStartPt->setVisible(false);
		innerStartPt->setScale(0.9);
		connect(innerStartPt, &EditPoint::pointMoved, this, &FractalCurve::setInnerStart);

		innerEndPt = new EditPoint();
		innerEndPt->setParentItem(this);
		innerEndPt->setVisible(false);
		innerEndPt->setScale(0.9);
		connect(innerEndPt, &EditPoint::pointMoved, this, &FractalCurve::setInnerEnd);

		endPt = new EditPoint();
		endPt->setParentItem(this);
		endPt->setVisible(false);
		endPt->setZValue(-1);
		connect(endPt, &EditPoint::pointMoved, this, &FractalCurve::setEnd);
	}

	FractalCurve::FractalCurve(QPointF s, QPointF e){
		setFlag(QGraphicsItem::ItemIsSelectable);
		initalizeEditPoints();
		initalizeNoise();
		setStart(s);
		setEnd(e);
	}

	FractalCurve::FractalCurve(QDataStream& is){
		setFlag(QGraphicsItem::ItemIsSelectable);
		initalizeEditPoints();
		initalizeNoise();
		is >> seed;
		QPointF start,end,midstart,midend;
		is >> start;
		is >> end;
		is >> midstart;
		is >> midend;
		setStart(start);
		setEnd(end);
		setInnerStart(midstart);
		setInnerEnd(midend);
		float f;
		is >> f;
		setFrequency(f);
	}

	qint8 FractalCurve::DrawableType(){
		return DrawableSerializeTypes::SerializeFractalCurve;
	}

	void FractalCurve::write(QDataStream& os){
		os << DrawableType();
		os << seed;

		os << mapToScene(start);
		os << mapToScene(end);
		os << mapToScene(innerStartPt->pos());
		os << mapToScene(innerEndPt->pos());

		os << getFrequency();
	}
	QPainterPath FractalCurve::shape() const
	{
		return generatePath();
	}

	QRectF FractalCurve::boundingRect() const
	{
		return childrenBoundingRect().marginsAdded(QMargins(5, 5, 5, 5));
	}

	QPainterPath FractalCurve::generatePath() const
	{
		qreal length = curve.length();

		// Figure out the number of points to render the line with
		const int POINTS = std::max(static_cast<int>(length / PERIOD), 2);
		// Dont draw really really short lines
		std::vector<float> curveNoise = std::vector<float>(((POINTS + 8) / 8) * 8); // Round to nearest multiple of 8, fastnoise runs better with it
		noise->GenUniformGrid2D(curveNoise.data(), 0, 0, ((POINTS + 8) / 8) * 8, 1, getFrequency(), seed);

		// Generate path
		QPainterPath path;
		if (length < 0.1){
			return path;
		}
		std::vector<float>::iterator ci = curveNoise.begin();
		for (auto i = curve.sweepLinearCurveIterator(POINTS); !i->isEmpty(); i->advance())
		{
			QPointF point = i->getPointTangent().point;
			QPointF perp = i->getPointTangent().tangent.transposed();
			perp.setY(-perp.y());
			//path.moveTo(point);
			point += Lipuma::normalize(perp) * ((*ci++) * HEIGHT);
			path.lineTo(point);
		}
		path.setElementPositionAt(path.elementCount()-1, end.x(), end.y());
		return path;
	}

	void FractalCurve::setStart(QPointF s)
	{
		// Store end location to keep end in place.
		QPointF gEnd = mapToScene(end);
		setPos(s);
		setEnd(gEnd);
	}

	void FractalCurve::setInnerStart(QPointF s)
	{
		innerStartPt->setPos(mapFromScene(s));
		curve.setPtB(innerStartPt->pos());
		prepareGeometryChange();
	}

	void FractalCurve::setInnerEnd(QPointF e)
	{
		innerEndPt->setPos(mapFromScene(e));
		curve.setPtC(innerEndPt->pos());
		prepareGeometryChange();
	}

	void FractalCurve::setEnd(QPointF e)
	{
		QPointF oldEnd = end;
		end = mapFromScene(e);
		endPt->setPos(end);
		curve.setPtD(endPt->pos());
		innerEndPt->setPos(innerEndPt->pos() + end - oldEnd);
		curve.setPtC(innerEndPt->pos());
		prepareGeometryChange();
	}

	QVariant FractalCurve::itemChange(GraphicsItemChange change, const QVariant &val)
	{
		QGraphicsItem::itemChange(change, val);
		if (change == ItemSelectedChange && scene())
		{
			startPt->setVisible(val.toBool());
			endPt->setVisible(val.toBool());
			innerStartPt->setVisible(val.toBool());
			innerEndPt->setVisible(val.toBool());
		}
		return val;
	}

	void FractalCurve::paint(QPainter *painter, const QStyleOptionGraphicsItem * /* option */, QWidget * /* widget */)
	{
		painter->setRenderHint(QPainter::Antialiasing, true);
		// Set highlight color if selected
		if (isSelected())
		{
			painter->setPen(QColor(255, 0, 0));
			painter->drawLine(start, innerStartPt->pos());
			painter->drawLine(innerEndPt->pos(), end);
		}

		QPainterPath path = generatePath();
		painter->drawPath(path);
		//painter->drawPath(shape());
		//painter->drawRect(boundingRect());
	}
}
