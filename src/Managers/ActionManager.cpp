#include"Managers/ActionManager.h"

#include<algorithm>
#include<iostream>
#include<utility>
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
    ActionManager::ActionManager(char **param, int paramCount) : 
        params(std::move(param)), paramCount(paramCount)
    {
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

        for (auto flag : flagVals)
        {
            if (flag.size() > 3 || isNumber(flag))
            {
                flg.value = flag;
                //cout<<"flag value "<<flg.value<<endl;
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
                //cout<<"flag "<<flag<<endl;
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
        
        for (auto i = 2; i < paramCount; ++i)
        {
            parsed.push_back(std::move(*(params + i)));
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
