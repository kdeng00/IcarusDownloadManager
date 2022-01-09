#include"Managers/ActionManager.h"

#include<algorithm>
#include<iostream>
#include<utility>
#include<cstring>

using std::cout;
using std::endl;
using std::string;
using std::string_view;
using std::vector;

using Models::Flags;
using Models::IcarusAction;

namespace Managers
{

#pragma region Constructors
ActionManager::ActionManager(char **param, int paramCount) : 
    params(std::move(param)), paramCount(paramCount)
{
    initialize();
}
#pragma endregion


#pragma region Functions
IcarusAction ActionManager::retrieveIcarusAction() const
{
    IcarusAction icarusAction;
    icarusAction.flags = flags;
    icarusAction.action = action;

    return icarusAction;
}


constexpr std::array<const char*, 12> ActionManager::supportedFlags() noexcept
{
    constexpr std::array<const char*, 12> allFlags{"-u", "-p", "-t", "-h", "-s",
            "-sd", "-sr", "-d", "-D", "-b", "-rt", "-nc"};

    return allFlags;
}


bool ActionManager::isNumber(string_view val) noexcept
{
    return !val.empty() && std::find_if(val.begin(), 
        val.end(), [](char c)
        {
            return !std::isdigit(c);
            }) == val.end();
}

void ActionManager::initialize()
{
    validateFlags();

    action = std::move(string{params[1]});
    transform(action.begin(), action.end(),
            action.begin(), ::tolower);
}
void ActionManager::validateFlags()
{
    cout<<"Validating flags"<<endl;

    auto flagVals = parsedFlags();

    Flags flg{};

    auto allSupportedFlags = supportedFlags();

    for (auto flag : flagVals)
    {
        if (flag.compare("-nc") == 0)
        {
            flg.flag = flag;
            flags.push_back(flg);
            continue;
        }
        if (flag.size() > 3 || isNumber(flag))
        {
            flg.value = flag;

            flags.push_back(flg);
            flg = Flags{};
            continue;
        }

        if (std::any_of(allSupportedFlags.begin(), allSupportedFlags.end(), 
            [&](const char *val)
            {
                return !flag.compare(val);
            }))
        {
            flg.flag = flag;
        }
        else
        {
            cout<<"Flag is not valid"<<endl;
            exit(1);
        }
    }
}

vector<string> ActionManager::parsedFlags()
{
    auto parsed = vector<string>();
    
    for (auto i = 2; i < paramCount; ++i)
    {
        const std::string flag(std::move(*(params + i)));
        parsed.push_back(std::move(flag));
    }

    return parsed;
}

#pragma region Testing
void ActionManager::printAction() noexcept
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
void ActionManager::printFlags() noexcept
{
    cout<<"\nPrinting flags..."<<endl;
    for (auto flag: flags)
    {
        cout<<"flag "<<flag.flag<<endl;
        cout<<"value "<<flag.value<<endl;
    }
}
#pragma endregion
#pragma endregion

}
