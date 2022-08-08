#ifndef CHECKS_H_
#define CHECKS_H_

#include<algorithm>
#include<string>
#include<cstdlib>
#include<ctype.h>

namespace Utilities
{
    class Checks
    {
    public:
        Checks() = delete;

        static auto isNumber(const std::string &val)
        {
            return !val.empty() && std::find_if(val.begin(), 
                val.end(), [](char c)
                {
                    return !std::isdigit(c);
                    }) == val.end();
        }

        // Note: Not implemented
        template<typename Item, typename Container, typename Func>
        static auto itemInContainer(const Container container, const Item item, Func func)
        {
            auto result = false;
            auto i = std::find_if(container.begin(), container.end(), [&](Item i)
            {
                return func(item, i);
            });

            if (i != container.end())
            {
                result = true;
            }

            return result;
        }

        template<typename Item, typename Container, typename Func>
        static auto itemIterInContainer(const Container &container, const Item &item, Func func)
        {
            auto result = false;
            // std::cout<<container<<"\n";
            auto ii = std::find_if(container.begin(), container.end(), [&](Item i)
            {
                // std::cout<<"iter "<<i<<" target "<<item<<"\n";
                return func(i, item);
            });

            if (ii == container.end())
            {
                // std::cout<<item<<" not found in container\n";
                ii = container.end();
            }

            return ii;
        }
    private:
    };
}

#endif



