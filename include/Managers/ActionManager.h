#ifndef ACTIONMANAGER_H_
#define ACTIONMANAGER_H_

#include<algorithm>
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
        constexpr std::array<const char*, 16> supportedFlags() noexcept
        {
            constexpr std::array<const char*, 16> allFlags{"-u", "-p", "-t", "-h", "-s",
                "-sd", "-sr", "-d", "-D", "-b", "-rt", "-nc",
                "-m", "-ca", "-smca", "-t"};

            return allFlags;
        }
        constexpr std::array<const char*, 4> supportedActions() noexcept;

        void initialize();
        void validateFlags();
        // Checks to see if the flag is valid
        template<typename Str>
        bool isValidFlag(const Str flag)
        {
            const auto flags = supportedFlags();
            const auto i = std::find_if(flags.begin(), flags.end(), [&](const Str &f)
            {
                return f.compare(flag) == 0 ? true : false;
            });

            auto result = i != flags.end() ? true : false;

            return result;
        }

        template<typename Str>
        bool doesFlagHaveValue(const Str flag)
        {
            const auto flags = parsedFlags();
            auto i = std::find_if(flags.begin(), flags.end(), [&](const Str &f)
            {
                return f.compare(flag) == 0 ? true : false;
            });

            if (i != flags.end())
            {
                if (++i != flags.end() && !isValidFlag<Str>(*i))
                {
                    return true;
                }
                else
                {
                    return false;
                }
            }
            else
            {
                return false;
            }
        }

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
