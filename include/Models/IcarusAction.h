#ifndef ICARUSACTION_H_
#define ICARUSACTION_H_

#include<string>
#include<algorithm>
#include<string_view>
#include<vector>
#include<iostream>

#include"Flags.h"

namespace Models
{
    class IcarusAction
    {
    public:
        std::string retrieveFlagValue(const std::string_view flag)
        {
            std::string value;

            const auto fg = std::find_if(flags.begin(), flags.end(), [&](Flags f)
            {
                return f.flag.compare(flag) == 0 ? true : false;
            });

            if (fg != flags.end())
            {
                value.assign(fg->value);
            }

            return value;
        }
        void print_action_and_flags() noexcept
        {
            std::cout<<"Action: "<<this->action<<"\n";
            std::cout<<"Flag count: "<<this->flags.size()<<"\n";
            
            for (const auto &flag : this->flags)
            {
                std::cout<<"flag "<<flag.flag<<" value "<<flag.value<<"\n";
            }

            std::cout<<"\n";
        }

        std::string action;
        std::vector<Flags> flags;
    };
}

#endif
