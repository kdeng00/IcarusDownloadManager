#ifndef ACTIONMANAGER_H_
#define ACTIONMANAGER_H_

#include<string>
#include<array>
#include<vector>

#include"Models/Flags.h"
#include"Models/IcarusAction.h"

namespace Managers
{
    class ActionManager
    {
    public:
        ActionManager(char**, int);

        Models::IcarusAction retrieveIcarusAction() const;
    private:
        bool isNumber(std::string);

        void initialize();
        void validateFlags();

        std::vector<std::string> parsedFlags();

        void printAction();
        void printFlags();

        std::string action;
        std::array<std::string, 4> supportedActions{
            "download", "upload", "retrieve", "delete"
        };
        std::array<std::string, 9> supportedFlags{
            "-u", "-p", "-t", "-h", "-s",
            "-d", "-D", "-b", "-rt"
        };
        std::vector<Models::Flags> flags;
        char **params;
        int paramCount;
    };
}

#endif
