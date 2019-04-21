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
#include<QPushButton>
#include<QTextEdit>
#include<QWidget>

#include"UI/CommonWindow.h"
#include"UI/AboutWindow.h"

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


        std::unique_ptr<QHBoxLayout> urlPortion;
        std::unique_ptr<QHBoxLayout> songPathPortion;

		std::unique_ptr<QWidget> mainWidgetQt;

		std::unique_ptr<QDockWidget> MainDockWidgetQt;

		std::unique_ptr<QPushButton> uploadSongQt;

		std::unique_ptr<QTextEdit> urlQt;
		std::unique_ptr<QTextEdit> sourceFilePathQt;

        std::unique_ptr<QLabel> urlLabel;
        std::unique_ptr<QLabel> songPath;

		std::unique_ptr<QMenu> fileMenuQt;
		std::unique_ptr<QMenu> editMenuQt;
		std::unique_ptr<QMenu> helpMenuQt;

		std::unique_ptr<QAction> closeApplicationQt;
		std::unique_ptr<QAction> aboutApplicationQt;

		std::unique_ptr<AboutWindow> aboutWindow;
	signals:
	private slots:
		void uploadSong();
		void exitApplication();
		void displaySoftwareInformation();
	};
}

#endif
