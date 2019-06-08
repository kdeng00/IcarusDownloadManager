#include"ActionManager.h"
#include<algorithm>
#include<iostream>
#include<exception>
#include<cstring>

using std::cout;
using std::endl;
using std::string;
using std::vector;

using Models::Flags;
using Models::IcarusAction;

namespace Managers
{
	#pragma
	ActionManager::ActionManager(char **param)
	{
		this->params = param;

		action = string{params[1]};
		transform(action.begin(), action.end(),
				action.begin(), ::tolower);

		initialize();
	}
	#pragma Constructors


	#pragma
	IcarusAction ActionManager::retrieveIcarusAction() const
	{
		auto icarusAction = IcarusAction{};
		icarusAction.flags = flags;
		icarusAction.action = action;

		return icarusAction;
	}
	vector<Flags> ActionManager::retrieveFlags() const
	{
		return flags;
	}

	string ActionManager::retrieveAction() const
	{
		return action;
	}

	bool ActionManager::isNumber(string val)
	{
		return !val.empty() && std::find_if(val.begin(), 
			val.end(), [](char c)
			{
				return !std::isdigit(c);
		       	}) == val.end();
	}

	void ActionManager::initialize()
	{
		initializeSupportedActions();
		validateAction();
		validateFlags();
	}
	void ActionManager::initializeSupportedActions()
	{
		supportedActions = vector<string>{
			"download", "delete",
			"retrieve", "upload"
		};
	}
	void ActionManager::initializeSupportedFlags()
	{
		supportedFlags = vector<string>{
			"-u", "-p", "-t", "-h", "-s",
			"-d", "-D", "-b"
		};
	}
	void ActionManager::validateAction()
	{
		cout<<"Validating action"<<endl;

		if (std::any_of(supportedActions.begin(), supportedActions.end(), 
			[&](string val)
			{
				return !val.compare(action);
			}))
		{
			cout<<"Action: "<<action<<" is valid"<<endl;
		}
		else
		{
			cout<<"Action is not valid"<<endl;
			exit(1);
		}
	}
	void ActionManager::validateFlags()
	{
		cout<<"Validating flags"<<endl;

		auto flagVals = parsedFlags();
		initializeSupportedFlags();

		Flags flg{};

		for (auto flag : flagVals)
		{
			//if (flag.size() > 3 ||  flg.flag.compare("-D"))
			if (flag.size() > 3 || isNumber(flag))
			{
				flg.value = flag;
				cout<<"flag value "<<flg.value<<endl;
				flags.push_back(flg);
				flg = Flags{};
				continue;
			}

			if (std::any_of(supportedFlags.begin(), supportedFlags.end(), 
				[&](string val)
				{
					return !val.compare(flag);
				}))
			{
				cout<<"flag "<<flag<<endl;
				flg.flag = flag;
			}
			else
			{
				cout<<"Action is not valid"<<endl;
				exit(1);
			}
		}
	}

	vector<string> ActionManager::parsedFlags()
	{
		auto parsed = vector<string>{};
		try
		{
			for (auto i = 2; true; ++i)
			{
				string val{*(params + i)};
				cout<<"Parsed flag "<<val<<endl;
				parsed.push_back(val);
			}
		}
		catch (std::exception e)
		{
			auto msg = e.what();
			cout<<"This happend: "<<msg<<endl<<endl;
		}

		return parsed;
	}

	#pragma
	void ActionManager::printAction()
	{
		if (action.empty())
		{
			printf("Action is empty\n");
		}
		else
		{
			cout<<"Action is "<<action<<endl;
		}
	}
	void ActionManager::printFlags(vector<string> flagVals)
	{
		if (flagVals.empty())
		{
			printf("Flags and values are empty\n");
		}
		else
		{
			printf("Printing flags and values..\n");
			for (auto flgVal : flagVals)
			{
				cout<<flgVal<<endl;
			}
		}
	}
	void ActionManager::printFlags()
	{
		cout<<"\nPrinting flags..."<<endl;
		for (auto flag: flags)
		{
			cout<<"flag "<<flag.flag<<endl;
			cout<<"value "<<flag.value<<endl;
		}
	}
	#pragma Testing
	#pragma Functions
}
