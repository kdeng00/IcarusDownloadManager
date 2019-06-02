#ifndef ACTIONMANAGER_H_
#define ACTIONMANAGER_H_

#include<string>
#include<vector>

#include"Models/Flags.h"
#include"Models/IcarusAction.h"

namespace Managers
{
	class ActionManager
	{
	public:
		ActionManager(char**);

		Models::IcarusAction retrieveIcarusAction() const;
		std::vector<Models::Flags> retrieveFlags() const;
		std::string retrieveAction() const;
	private:
		void initialize();
		void initializeSupportedActions();
		void initializeSupportedFlags();
		void validateAction();
		void validateFlags();

		std::vector<std::string> parsedFlags();

		void printAction();
		void printFlags(std::vector<std::string>);
		void printFlags();

		std::string action;
		std::vector<std::string> supportedActions;
		std::vector<std::string> supportedFlags;
		std::vector<Models::Flags> flags;
		char **params;
	};
}

#endif
