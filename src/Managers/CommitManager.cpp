#include"Managers/CommitManager.h"

#include<iostream>

#include"Models/API.h"
#include"Models/Song.h"
#include"Models/Token.h"
#include"Parsers/APIParser.h"
#include"Syncers/Delete.h"
#include"Syncers/Download.h"
#include"Syncers/Upload.h"

#include"Managers/TokenManager.h"
#include"Managers/UserManager.h"

using std::cout;
using std::endl;
using std::map;
using std::string;

using Managers::TokenManager;
using Managers::UserManager;
using Models::API;
using Models::Song;
using Models::Token;
using Parsers::APIParser;
using Models::IcarusAction;
using Syncers::Delete;
using Syncers::Download;
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
				deleteSong();
				break;
			case downloadAct:
				downloadSong();
				break;
			case retrieveAct: // No plans to imeplement
				break;
			case uploadAct:
				uploadSong();
				break;
		}
	}

    Token CommitManager::parseToken(API api)
    {
        cout<<"fetching token"<<endl;
		UserManager usrMgr{icaAction};
		auto user = usrMgr.retrieveUser();

		TokenManager tk{user, api};
        
        return tk.requestToken();
    }

	void CommitManager::initializeMapActions()
	{
		mapActions = map<string, ActionValues>{
			{"delete", deleteAct}, {"download", downloadAct},
			{"retrieve", retrieveAct},
			{"upload", uploadAct}
		};
	}
	void CommitManager::deleteSong()
	{
		APIParser apiPrs{icaAction};
		auto api = apiPrs.retrieveAPI();

		auto token = parseToken(api);

		Song song{};

		for (auto arg : icaAction.flags)
		{
			auto flag = arg.flag;
			auto value = arg.value;

			if (flag.compare("-D") == 0)
			{
				song.id = atoi(value.c_str());
			}
		}

		Delete del{api};
		cout<<"Deleting song..."<<endl;
		del.deleteSong(token, song);
	}
	void CommitManager::downloadSong()
	{
		cout<<"Starting downloading process..."<<endl;

		APIParser apiPrs{icaAction};
		auto api = apiPrs.retrieveAPI();

		auto token = parseToken(api);

		Song song{};

		for (auto arg : icaAction.flags)
		{
			auto flag = arg.flag;
			auto value = arg.value;

			if (flag.compare("-d") == 0)
			{
				song.songPath.assign(arg.value);
			}
			if (flag.compare("-b") == 0)
			{
				song.id = atoi(value.c_str());
			}
		}

		Download dnld{api};
        cout<<"downloading song"<<endl;
		dnld.downloadSong(token, song);
	}
	void CommitManager::uploadSong()
	{
		APIParser apiPrs{icaAction};
		auto api = apiPrs.retrieveAPI();

		auto token = parseToken(api);

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
		cout<<"Uploading song..."<<endl;
		upld.uploadSong(token, song);
	}
	#pragma Functions
}
