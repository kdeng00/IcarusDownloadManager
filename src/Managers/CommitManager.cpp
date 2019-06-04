#include"CommitManager.h"

#include<iostream>

#include"Syncers/Upload.h"

#include"TokenManager.h"
#include"UserManager.h"

using std::cout;
using std::endl;
using std::map;
using std::string;

using Managers::TokenManager;
using Managers::UserManager;
using Models::IcarusAction;
using Syncers::Upload;

namespace Managers
{
	#pragma
	CommitManager::CommitManager(IcarusAction icaAct)
	{
		icaAction = icaAct;
		initializeMapActions();
	}
	#pragma Constructors;


	#pragma
	void CommitManager::commitAction()
	{
		auto action = icaAction.action;
		cout<<"Commitning "<<action<<" action"<<endl;
		switch (mapActions[action])
		{
			case deleteAct:
				break;
			case downloadAct:
				break;
			case retrieveAct:
				break;
			case uploadAct:
				uploadSong();
				break;
		}
	}

	void CommitManager::initializeMapActions()
	{
		mapActions = map<string, ActionValues>{
			{"delete", deleteAct}, {"download", downloadAct},
			{"retrieve", retrieveAct},
			{"upload", uploadAct}
		};
	}
	void CommitManager::uploadSong()
	{
		cout<<"Uploading song..."<<endl;
		UserManager usrMgr{icaAction};
		auto user = usrMgr.retrieveUser();

		TokenManager tk{user};
		auto token = tk.requestToken();

		string songPath{};

		for (auto arg : icaAction.flags)
		{
			auto flag = arg.flag;
			auto value = arg.value;

			if (flag.compare("-s") == 0)
			{
				songPath.assign(arg.value);
			}
		}

		Upload upld{};
		upld.uploadSong(token, songPath);

		// TODO: Implement functionality for uploading
		//
		// Need to the following:
		// 1. Parse the login credentials from the Flag model x
		// 2. Retrieve a token -- implement token management x
		// 3. Parse song path from the Flag model x Make a parser class
		// 4. Parse the HTTP API endpoint Make parser class
		// 5. Upload the song x
	}
	#pragma Functions
}
