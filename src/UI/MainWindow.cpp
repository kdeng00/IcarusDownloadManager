#include"UI/MainWindow.h"

#include<iostream>
#include<string>

#include"Models/UploadForm.h"
#include"Syncers/Upload.h"
#include"Utilities/Conversions.h"

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
        aboutWindow = unique_ptr<AboutWindow>{new AboutWindow};
    }


    void MainWindow::configureDownloadSection()
    {
    }
    void MainWindow::configureUploadSection()
    {
        uploadSongQt = unique_ptr<QPushButton>{new QPushButton(tr("Upload"))};
        urlQt = unique_ptr<QTextEdit>{new QTextEdit()};
        sourceFilePathQt = unique_ptr<QTextEdit>{new QTextEdit()};

            urlLabel = unique_ptr<QLabel>{new QLabel(tr("URL"))};
            songPath = unique_ptr<QLabel>{new QLabel(tr("Song Path"))};

            urlPortion = unique_ptr<QHBoxLayout>{new QHBoxLayout};
            songPathPortion = unique_ptr<QHBoxLayout>{new QHBoxLayout};

            urlPortion.get()->addWidget(urlLabel.get());
            urlPortion.get()->addWidget(urlQt.get());

            songPathPortion->addWidget(songPath.get());
            songPathPortion->addWidget(sourceFilePathQt.get());

        subLayoutOneQt = unique_ptr<QVBoxLayout>{new QVBoxLayout};
            subLayoutOneQt.get()->addLayout(urlPortion.get());
            subLayoutOneQt->addLayout(songPathPortion.get());
        mainLayoutQt.get()->addLayout(subLayoutOneQt.get());
    }
    void MainWindow::configureWindowDimensions()
    {
        windowWidth = 450;
        windowHeight = 450;
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
        QObject::connect(closeApplicationQt.get(), SIGNAL(triggered()), this, 
                        SLOT(exitApplication()));
        QObject::connect(aboutApplicationQt.get(), SIGNAL(triggered()), this, 
                        SLOT(displaySoftwareInformation()));
        QObject::connect(windowComboBox.get(), SIGNAL(activated(int)),
                                this, SLOT(setCurrentIndex(int)));
    }
    void MainWindow::createMenus()
    {
        fileMenuQt = unique_ptr<QMenu>{menuBar()->addMenu(tr("File"))};
        editMenuQt = unique_ptr<QMenu>{menuBar()->addMenu(tr("Edit"))};
        helpMenuQt = unique_ptr<QMenu>{menuBar()->addMenu(tr("Help"))};

        closeApplicationQt = unique_ptr<QAction>{new QAction(new QObject(nullptr))};
        closeApplicationQt->setText("Exit Application");

        aboutApplicationQt = unique_ptr<QAction>{new QAction(new QObject(nullptr))};
        aboutApplicationQt->setText("About");

        fileMenuQt->addAction(closeApplicationQt.get());
        helpMenuQt->addAction(aboutApplicationQt.get());

    }
    void MainWindow::setupMainWidget()
    {
        mainWidgetQt = unique_ptr<QWidget>{new QWidget};

        windowComboBox = unique_ptr<QComboBox>{new QComboBox};
        setupWindowLists();


        stackLayout = unique_ptr<QVBoxLayout>{new QVBoxLayout};
        stackLayout->addWidget(windowComboBox.get());

        uploadSongWidgetQt = unique_ptr<QWidget>{new QWidget};
        uploadSongWidgetQt->setLayout(mainLayoutQt.get());

        stackLayout->addWidget(uploadSongWidgetQt.get());

        mainWidgetQt->setLayout(stackLayout.get());
    }
    void MainWindow::setupMainWindow()
    {
        configureWindowDimensions();

        mainLayoutQt = unique_ptr<QVBoxLayout>{new QVBoxLayout};
        widgetStack = unique_ptr<QStackedWidget>{new QStackedWidget};

        configureUploadSection();

        setupMainWidget();

        widgetStack->addWidget(mainWidgetQt.get());

        MainDockWidgetQt = unique_ptr<QDockWidget>{new QDockWidget};
        MainDockWidgetQt.get()->setWindowTitle(tr("Music Manager"));
        MainDockWidgetQt->setWidget(widgetStack.get());
        MainDockWidgetQt.get()->setFeatures(QDockWidget::NoDockWidgetFeatures);

        setCentralWidget(MainDockWidgetQt.get());

        createMenus();

        configureWindowProperties();

        connections();
    }
    void MainWindow::setupWindowLists()
    {
        windowComboBox->addItem(tr("Upload song"));
        windowComboBox->addItem(tr("Download song"));
        windowComboBox->addItem(tr("Display all songs"));
        windowComboBox->addItem(tr("Display songs"));
    }


    void MainWindow::exitApplication()
    {
        exit(0);
    }
    void MainWindow::displaySoftwareInformation()
    {
        aboutWindow->show();
    }
    void MainWindow::setCurrentIndex(int index)
    {
        cout<<"index "<<index<<endl;
        QString qText = windowComboBox->itemText(index);
        auto cnvert = Utilities::Conversions(qText);
        auto convertedStr = cnvert.convertQStringToString();
        cout<<"item text"<<endl;
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
