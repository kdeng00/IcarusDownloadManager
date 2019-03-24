#include"MainWindow.h"

#include<iostream>
#include<string>

#include"Models/UploadForm.h"
#include"Syncers/Upload.h"

using std::cout;
using std::endl;
using std::string;
using std::unique_ptr;

using Models::UploadForm;
using Syncers::Upload;

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
		uploadSongQt = unique_ptr<QPushButton>{new QPushButton(tr("Upload"))};
		urlQt = unique_ptr<QTextEdit>{new QTextEdit()};
		sourceFilePathQt = unique_ptr<QTextEdit>{new QTextEdit()};

		subLayoutOneQt = unique_ptr<QVBoxLayout>{new QVBoxLayout};
		subLayoutOneQt.get()->addWidget(urlQt.get());
		subLayoutOneQt.get()->addWidget(sourceFilePathQt.get());
		subLayoutOneQt.get()->addWidget(uploadSongQt.get());
		mainLayoutQt.get()->addLayout(subLayoutOneQt.get());
	}
	void MainWindow::configureWindowDimensions()
	{
		windowWidth = 400;
		windowHeight = 400;
	}
	void MainWindow::configureWindowProperties()
	{
		setWindowTitle("IcarusDownloadManager");
		setFixedHeight(windowHeight);
		setFixedWidth(windowWidth);
	}
	void MainWindow::connections()
	{
		QObject::connect(uploadSongQt.get(), SIGNAL(clicked()), this, SLOT(uploadSong()));
	}
	void MainWindow::createMenus()
	{
		fileMenuQt = unique_ptr<QMenu>{menuBar()->addMenu(tr("File"))};
		editMenuQt = unique_ptr<QMenu>{menuBar()->addMenu(tr("Edit"))};

		closeApplicationQt = unique_ptr<QAction>{new QAction(new QObject(nullptr))};
		closeApplicationQt.get()->setText("Exit Application");

		/**
		fileMenu.get()->addAction(closeApplication.get());
		editMenu.get()->addAction(keyEdit.get());
		editMenu.get()->addAction(passwordManage.get());
		*/

	}
	void MainWindow::setupMainWindow()
	{
		configureWindowDimensions();
		mainLayoutQt = unique_ptr<QVBoxLayout>{new QVBoxLayout};

		configureUploadSection();

		mainWidgetQt = unique_ptr<QWidget>{new QWidget};
		mainWidgetQt.get()->setLayout(mainLayoutQt.get());

		MainDockWidgetQt = unique_ptr<QDockWidget>{new QDockWidget};
		MainDockWidgetQt.get()->setWindowTitle(tr("Music Manager"));
		MainDockWidgetQt.get()->setWidget(mainWidgetQt.get());
		MainDockWidgetQt.get()->setFeatures(QDockWidget::NoDockWidgetFeatures);
		setCentralWidget(MainDockWidgetQt.get());

		createMenus();

		configureWindowProperties();

		connections();
	}


	void MainWindow::uploadSong()
	{
		uploadSongQt->setEnabled(false);

		string url = urlQt->toPlainText().toUtf8().constData();
		string filePath = sourceFilePathQt->toPlainText().toUtf8().constData();
		cout<<"URL endpoint: "<<url<<endl;
		cout<<"Music file path: "<<filePath<<endl;
		UploadForm formData{url, filePath};

		Upload upld{formData};
		upld.uploadSong();

		uploadSongQt->setEnabled(true);
	}
}
