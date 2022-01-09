#ifndef APIPARSER_H_
#define APIPARSER_H_

#include"Models/API.h"
#include"Models/IcarusAction.h"

namespace Parsers
{

class APIParser
{
public:
    APIParser(Models::IcarusAction);

    Models::API retrieveAPI() const;
private:
    void parseAPI();

    Models::API api;
    Models::IcarusAction icaAct;
};

}

#endif
