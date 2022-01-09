#ifndef TOKEN_H_
#define TOKEN_H_

#include<string>

namespace Models
{

struct Token
{
    std::string accessToken;
    std::string tokenType;
    int expiration;
};

}

#endif
