#include"CommitManager.h"

#include<iostream>

#include"Models/API.h"
#include"Models/Song.h"
#include"Parsers/APIParser.h"
#include"Syncers/Upload.h"

#include"TokenManager.h"
#include"UserManager.h"

using std::cout;
using std::endl;
using std::map;
using std::string;

using Managers::TokenManager;
using Managers::UserManager;
using Models::API;
using Models::Song;
using Parsers::APIParser;
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
				downloadSong();
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
	void CommitManager::downloadSong()
	{
		// TODO: Implement this functionality
	}
	void CommitManager::uploadSong()
	{
		cout<<"Uploading song..."<<endl;
		UserManager usrMgr{icaAction};
		auto user = usrMgr.retrieveUser();

		APIParser apiPrs{icaAction};
		auto api = apiPrs.retrieveAPI();

		TokenManager tk{user, api};
		auto token = tk.requestToken();

		Song song{};

		for (auto arg : icaAction.flags)
		{
			auto flag = arg.flag;
			auto value = arg.value;

			if (flag.compare("-s") == 0)
			{
				song.songPath.assign(arg.value);
			}
		}

		Upload upld{api};
		upld.uploadSong(token, song);
	}
	#pragma Functions
}
