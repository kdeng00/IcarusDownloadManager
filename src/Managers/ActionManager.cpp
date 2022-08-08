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



void ActionManager::initialize()
{
    validateFlags();

    action = std::move(string{params[1]});
    transform(action.begin(), action.end(),
            action.begin(), ::tolower);
}
void ActionManager::validateFlags()
{
    cout<<"Validating flags\n";

    const auto flagVals = parsedFlags();

    for (auto flag = flagVals.begin(); flag != flagVals.end(); ++flag)
    {
        Flags flg;
        cout<<"Value: "<<*flag<<"\n";

        if (isValidFlag<string>(*flag) && doesFlagHaveValue<string>(*flag))
        {
            cout<<"Flag has value\n";
            flg.flag = *flag;
            flg.value = *(++flag);
        }
        else if (isValidFlag<string>(*flag))
        {
            cout<<"Flag does not have a value\n";
            flg.flag = *flag;
        }
        else
        {
            cout<<"Flag "<<*flag<<" is not valid"<<endl;
            exit(1);
        }

        flags.emplace_back(std::move(flg));
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
