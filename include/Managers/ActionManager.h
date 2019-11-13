#ifndef ACTIONMANAGER_H_
#define ACTIONMANAGER_H_

#include<string>
#include<string_view>
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
        constexpr std::array<const char*, 12> supportedFlags() noexcept;
        constexpr std::array<const char*, 4> supportedActions() noexcept;

        bool isNumber(const std::string_view) noexcept;

        void initialize();
        void validateFlags();

        std::vector<std::string> parsedFlags();

        void printAction() noexcept;
        void printFlags() noexcept;

        std::string action;
        
        std::vector<Models::Flags> flags;

        char **params;
        int paramCount;
    };
}

#endif
