#include "Utilities/Conversions.h"

#include <iostream>

using std::string;
using std::unique_ptr;

namespace Utilities
{

Conversions::Conversions()
{
    initializeValues();
}

void Conversions::initializeValues()
{
}
template <typename T>
void Conversions::printValue(T val)
{
    std::cout<<"going to print value\n";
    std::cout<<val<< "\n";
}

}
