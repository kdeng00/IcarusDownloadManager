#include"UserManager.h"

#include<iostream>
#include<string>
#include<vector>

using std::string;
using std::vector;

using Models::IcarusAction;
using Models::User;

namespace Managers
{
	#pragma
	UserManager::UserManager(User user)
	{
		this->user = user;
	}
	UserManager::UserManager(const IcarusAction icaAct)
	{
		this->icaAction = icaAct;
		this->user = User{};
		parseUserFromActions();
	}
	#pragma Constructors


	#pragma
	User UserManager::retrieveUser() const
	{
		return user;
	}

	void UserManager::parseUserFromActions()
	{
		auto args = icaAction.flags;

		for (auto arg : args)
		{
			auto flag = arg.flag;
			if (flag.compare("-u") == 0)
			{
				user.username = arg.value;
			}
			if (flag.compare("-p") == 0)
			{
				user.password = arg.value;
			}
		}
	}
	#pragma Functions
}
