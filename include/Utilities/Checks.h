#ifndef CHECKS_H_
#define CHECKS_H_

#include<algorithm>
#include<cstdlib>
#include<ctype.h>

namespace Utilities
{
    class Checks
    {
    public:
        Checks() = delete;

        static bool isNumber(const std::string &val)
        {
            return !val.empty() && std::find_if(val.begin(), 
                val.end(), [](char c)
                {
                    return !std::isdigit(c);
                    }) == val.end();
        }
    private:
    };
}

#endif



