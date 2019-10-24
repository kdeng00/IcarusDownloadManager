#include"Managers/CommitManager.h"

#include<iostream>

#include"Models/API.h"
#include"Models/Song.h"
#include"Models/Token.h"
#include"Parsers/APIParser.h"
#include"Syncers/Delete.h"
#include"Syncers/Download.h"
#include"Syncers/RetrieveRecords.h"
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
using Syncers::RetrieveRecords;
using Syncers::Upload;

namespace Managers
{
	#pragma
	CommitManager::CommitManager(IcarusAction& icaAct) : icaAction(std::move(icaAct))
	{ }
	#pragma Constructors;


	#pragma
	void CommitManager::commitAction()
	{
		auto action = icaAction.action;
		cout<<"Commiting "<<action<<" action"<<endl;
		switch (mapActions[action])
		{
			case deleteAct:
				deleteSong();
				break;
			case downloadAct:
				downloadSong();
				break;
			case retrieveAct:
                retrieveObjects();
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
    void CommitManager::retrieveObjects()
    {
        cout<<"Starting retrieve process..."<<endl;
        
        APIParser apiPrs{icaAction};
        auto api = apiPrs.retrieveAPI();

        auto token = parseToken(api);
        RetrieveTypes retrieveType;

        for (auto arg : icaAction.flags)
        {
            auto flag = arg.flag;
            auto value = arg.value;

            if (flag.compare("-rt") == 0)
            {
                if (value.compare("songs") == 0)
                {
                    retrieveType = RetrieveTypes::songs;
                    break;
                }
            }
        }

        RetrieveRecords songs{api, token};
        songs.retrieve(retrieveType);

    }
	void CommitManager::uploadSong()
	{
        bool uploadSingleSong = true;
        bool recursiveDirectory = false;
        string songDirectory;
		APIParser apiPrs{icaAction};
		auto api = apiPrs.retrieveAPI();

		auto token = parseToken(api);

		Song song;

		for (auto& arg : icaAction.flags)
		{
			auto flag = arg.flag;
			auto value = arg.value;

			if (flag.compare("-s") == 0)
			{
				song.songPath.assign(arg.value);
			}
            else if (flag.compare("-sd") == 0)
            {
                songDirectory = value;
                uploadSingleSong = false;
            } 
            else if (flag.compare("-sr") == 0)
            {
                songDirectory = value;
                uploadSingleSong = false;
                recursiveDirectory = true;
            }
		}

		Upload upld{api};
        if (uploadSingleSong)
        {
		    cout<<"Uploading song..."<<endl;
		    upld.uploadSong(token, song);
        }
        else
        {
            cout<<"Uploading songs from " << songDirectory << endl;
            upld.uploadSongsFromDirectory(token, songDirectory, recursiveDirectory);
        }
	}
	#pragma Functions
}
