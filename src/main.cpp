#include <QApplication>
#include <QMainWindow>
#include <QPainter>
#include <QMenu>
#include <QAction>
#include <QDockWidget>
#include <QTextEdit>
#include <QtWidgets>
#include "widget/canvas.hpp"

#include "widget/toolSelector.hpp"
#include "widget/propertiesMenu.hpp"
#include "file/serializer.hpp"

int main (int argc, char **argv){
	QApplication a(argc, argv);
	QGraphicsScene *scene = new QGraphicsScene();
	Lipuma::Canvas *canvas = new Lipuma::Canvas(scene);
	QMainWindow *mainWin = new QMainWindow();
	mainWin->setCentralWidget(canvas);
	QMenu* fileMenu = mainWin->menuBar()->addMenu("File");

	QAction* saveAction = fileMenu->addAction("&Save As");
	QObject::connect(saveAction, &QAction::triggered, fileMenu,  [canvas, mainWin](){
		QString s = QFileDialog::getSaveFileName(mainWin, "Open Image", "$HOME/Documents/", "Lipuma Files (*.lpm)");
		Lipuma::SerializeScene(canvas->scene(),s);
	});

	QAction* loadAction = fileMenu->addAction("&Open Canvas");
	QObject::connect(loadAction, &QAction::triggered, mainWin,  [mainWin](){
		QString s = QFileDialog::getOpenFileName(mainWin, "Open Image", "$HOME/Documents/", "Lipuma Files (*.lpm)");
		QGraphicsScene* scene = Lipuma::LoadScene(s);
		if (scene) // Can return nullptr
		{
			dynamic_cast<Lipuma::Canvas*>(mainWin->centralWidget())->scene()->clear();
			dynamic_cast<Lipuma::Canvas*>(mainWin->centralWidget())->setScene(scene);
		}
		else
		{
			QMessageBox::warning(mainWin, "File Load Error", "File failed to load.");
		}
	});

	QDockWidget *dock = new QDockWidget();
	Lipuma::ToolSelector *selector = new Lipuma::ToolSelector(dock);
	QObject::connect(selector, &Lipuma::ToolSelector::toolSelected, canvas, &Lipuma::Canvas::toolSelected);
	dock->setWidget(selector);
	mainWin->addDockWidget(Qt::LeftDockWidgetArea, dock);

	QDockWidget *rightDock = new QDockWidget();
	Lipuma::PropertiesMenu *menu = new Lipuma::PropertiesMenu(canvas, rightDock);
	QObject::connect(scene, &QGraphicsScene::selectionChanged, menu, &Lipuma::PropertiesMenu::selectionUpdated);
	mainWin->addDockWidget(Qt::RightDockWidgetArea, rightDock);
	rightDock->setWidget(menu);

	mainWin->show();
	return a.exec();
}
