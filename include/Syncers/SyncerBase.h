#ifndef SYNCERBASE_H_
#define SYNCERBASE_H_

#include<string>

#include"Models/API.h"

namespace Syncers
{

class SyncerBase
{
protected:
    Models::API api;
    const int OK = 200;
    const int UNAUTHORIZED = 401;
    const int NOTFOUND = 404;

    enum class Result 
    {
        OK = 200,
        UNAUTHORIZED = 401,
        NOTFOUND = 404
    };
};

}

#endif
