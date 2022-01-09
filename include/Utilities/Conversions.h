#ifndef CONVERSIONS_H_
#define CONVERSIONS_H_

#include<memory>
#include<string>

namespace Utilities
{

class Conversions
{
public:
    Conversions();

    static void toLowerChar(char &c)
    {
        if (std::isalpha(c))
        {
            c = std::tolower(c);
        }
    }

    void initializeValues();

    template<typename T>
    void printValue(T val);
private:
};

}

#endif
