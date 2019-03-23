#include<iostream>
#include<string>

#include<QApplication>

#include"UI/MainWindow.h"

using std::cin;
using std::cout;
using std::endl;
using std::string;

using UI::MainWindow;

string songPath{};
string newSongPath;

int main(int argc, char** argv)
{
	cout<<"Argument size: "<<argc<<endl;
	switch(argc)
	{
		case 0:
			break;
		case 1:
			break;
		case 2:
			break;
		case 3:
			songPath = argv[1];
			newSongPath = argv[2];
			break;
		default:
			break;
	}

	QApplication app{argc, argv};

	MainWindow icarusMgr{};
	icarusMgr.show();


	return app.exec();
}
