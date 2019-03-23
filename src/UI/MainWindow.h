#ifndef MAINWINDOW_H_
#define MAINWINDOW_H_

#include<iostream>
#include<memory>

#include<QAction>
#include<QDialog>
#include<QDockWidget>
#include<QLabel>
#include<QMenu>
#include<QMenuBar>
#include<QMainWindow>
#include<QTextEdit>
#include<QWidget>

#include"UI/CommonWindow.h"

namespace UI
{
	class MainWindow: public QMainWindow, public CommonWindow
	{
		Q_OBJECT
	public:
		MainWindow();
		~MainWindow() = default;
	private:
		void configureDownloadSection();
		void configureUploadSection();
		void configureWindowDimensions();
		void configureWindowProperties();
		void connections();
		void createMenus();
		void setupMainWindow();

		std::unique_ptr<QVBoxLayout> buttonLayoutQt;

		std::unique_ptr<QWidget> buttonWidgetQt;

		std::unique_ptr<QDockWidget> buttonDockWidgetQt;
		std::unique_ptr<QDockWidget> cryptionAreaQt;

		std::unique_ptr<QTextEdit> sourceFilePathQt;

		std::unique_ptr<QMenu> fileMenuQt;
		std::unique_ptr<QMenu> editMenuQt;
		std::unique_ptr<QAction> closeApplicationQt;
	signals:
	private slots:
	};
}

#endif
