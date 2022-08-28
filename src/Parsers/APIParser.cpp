#include "Parsers/APIParser.h"

#include <iostream>

using std::cout;
using std::endl;

using Models::API;
using Models::IcarusAction;

namespace Parsers
{

#pragma region Constructors
APIParser::APIParser(IcarusAction icaAct) : icaAct(icaAct)
{
    api = API{};
    parseAPI();
}
#pragma endregion


#pragma region Functions
API APIParser::retrieveAPI() const
{
    return api;
}

void APIParser::parseAPI()
{
    auto flags = icaAct.flags;
    cout << "Parsing api" << endl;

    for (auto i =0; i < flags.size(); ++i)
    {
        auto arg = flags[i].flag;
        auto value = flags[i].value;

        if (arg.compare("-h") == 0)
        {
            api.url = (value[value.size()-1] == '/') ? value : value + "/";
            break;
        }
    }

    // NOTE: For now I will hard code
    // the api version since I am only
    // on version 1
    api.version = "v1";
}
#pragma endregion

}
