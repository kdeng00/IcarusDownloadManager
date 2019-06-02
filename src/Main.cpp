#include<iostream>
#include<string>

#include"Managers/ActionManager.h"

using std::cin;
using std::cout;
using std::endl;
using std::string;

using Managers::ActionManager;

string songPath{};
string newSongPath{};

int main(int argc, char** argv)
{
	if (argc <= 1)
	{
		cout<<"No actions provided"<<endl;
		return 1;
	}

	ActionManager actMgr{argv};


	return 0;
}
