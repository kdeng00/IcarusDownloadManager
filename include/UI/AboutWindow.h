#ifndef ABOUTWINDOW_H_
#define ABOUTWINDOW_H_

#include<memory>

#include<QDialog>
#include<QDockWidget>
#include<QLabel>
#include<QPushButton>
#include<QWidget>

#include"UI/CommonWindow.h"

namespace UI
{
    class AboutWindow: public QDialog, public CommonWindow
    {
        Q_OBJECT
    public:
        AboutWindow(QWidget* parent=0);
        ~AboutWindow() = default;

    private:
        void connections();
        void setupWindow();

        std::unique_ptr<QLabel> appName;

    private slots:
        void closeWindow();
    };
}

#endif
