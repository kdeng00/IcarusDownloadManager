#include "UI/AboutWindow.h"

using std::unique_ptr;

namespace UI
{
#pragma region Constructors
AboutWindow::AboutWindow(QWidget* parent): QDialog(parent)
{
    setupWindow();
}
#pragma endregion


#pragma region Functions
void AboutWindow::setupWindow()
{
    windowWidth = 250;
    windowHeight = 300;

    mainLayoutQt = unique_ptr<QVBoxLayout>{new QVBoxLayout};

    appName = unique_ptr<QLabel>{new QLabel(tr("IcarusDownloadManager"))};
    actionButtonQt = unique_ptr<QPushButton>{new QPushButton(tr("Close"))};

    mainLayoutQt->addWidget(appName.get());
    mainLayoutQt->addWidget(actionButtonQt.get());


    setFixedWidth(windowWidth);
    setFixedHeight(windowHeight);

    setLayout(mainLayoutQt.get());

    setWindowTitle("About");

    connections();
}
void AboutWindow::connections()
{
    QObject::connect(actionButtonQt.get(), SIGNAL(clicked()), this, 
                    SLOT(closeWindow()));
}


void AboutWindow::closeWindow()
{
    this->hide();   
}
#pragma endregion

}
