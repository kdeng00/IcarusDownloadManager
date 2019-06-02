#include"CommitManager.h"

#include<iostream>

using std::cout;
using std::endl;
using std::map;
using std::string;

using Models::IcarusAction;

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
		// TODO: Implement functionality for uploading
		//
		// Need to the following:
		// 1. Parse the login credentials from the Flag model
		// 2. Retrieve a token -- implement token management
		// 3. Parse song path from the Flag model
		// 4. Parse the HTTP API endpoint
		// 5. Upload the song
	}
	#pragma Functions
}
