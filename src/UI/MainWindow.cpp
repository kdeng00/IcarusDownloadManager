#include"MainWindow.h"

using std::unique_ptr;

namespace UI
{
	MainWindow::MainWindow() 
	{

		setupMainWindow();
   	}


	void MainWindow::configureDownloadSection()
	{
	}
	void MainWindow::configureUploadSection()
	{
	}
	void MainWindow::configureWindowDimensions()
	{
		windowWidth = 600;
		windowHeight = 600;
	}
	void MainWindow::configureWindowProperties()
	{
		setWindowTitle("IcarusDownloadManager");
		setFixedHeight(windowHeight);
		setFixedWidth(windowWidth);
	}
	void MainWindow::connections()
	{
	
	}
	void MainWindow::createMenus()
	{
		fileMenuQt = unique_ptr<QMenu>{menuBar()->addMenu(tr("File"))};
		editMenuQt = unique_ptr<QMenu>{menuBar()->addMenu(tr("Edit"))};

		closeApplicationQt = unique_ptr<QAction>{new QAction(new QObject(nullptr))};
		closeApplicationQt.get()->setText("Exit Application");

		/**
		keyEdit = unique_ptr<QAction>{new QAction(new QObject(nullptr))};
		keyEdit.get()->setText("Key Management");
		passwordManage = unique_ptr<QAction>{new QAction{new QObject{nullptr}}};
		passwordManage.get()->setText("PasswordManagement");

		fileMenu.get()->addAction(closeApplication.get());
		editMenu.get()->addAction(keyEdit.get());
		editMenu.get()->addAction(passwordManage.get());
		*/

	}
	void MainWindow::setupMainWindow()
	{
		configureWindowDimensions();
		actionButtonQt = unique_ptr<QPushButton>{new QPushButton(tr("upload"))};
		selectionBoxQt = unique_ptr<QComboBox>{new QComboBox{}};

		buttonLayoutQt = unique_ptr<QVBoxLayout>{new QVBoxLayout};
		buttonWidgetQt = unique_ptr<QWidget>{new QWidget};
		buttonDockWidgetQt = unique_ptr<QDockWidget>{new QDockWidget};
		buttonDockWidgetQt.get()->setWindowTitle(tr("Music Manager"));
		buttonLayoutQt.get()->addWidget(selectionBoxQt.get());
		buttonLayoutQt.get()->addWidget(actionButtonQt.get());
		buttonWidgetQt.get()->setLayout(buttonLayoutQt.get());
		buttonDockWidgetQt.get()->setWidget(buttonWidgetQt.get());
		buttonDockWidgetQt.get()->setFeatures(QDockWidget::NoDockWidgetFeatures);
		setCentralWidget(buttonDockWidgetQt.get());

		cryptionAreaQt = unique_ptr<QDockWidget>{new QDockWidget(tr("Cryption"))};
		cryptionAreaQt.get()->setFeatures(QDockWidget::NoDockWidgetFeatures);
		addDockWidget(Qt::LeftDockWidgetArea, cryptionAreaQt.get());

		createMenus();


		configureWindowProperties();
		actionButtonQt.get()->setEnabled(false);

		connections();
	}
}
