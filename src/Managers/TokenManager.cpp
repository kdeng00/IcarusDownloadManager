#include"Managers/TokenManager.h"

#include<iostream>

#include<cpr/cpr.h>
#include<nlohmann/json.hpp>

using std::cout;
using std::endl;

using json = nlohmann::json;

using Managers::TokenManager;
using Models::API;
using Models::Token;
using Models::User;

namespace Managers
{

#pragma region Constructors
TokenManager::TokenManager(const User& user)
{
    this->user = user;
}
TokenManager::TokenManager(const User& user, API& api)
{
    this->user = user;
    this->api = api;
    this->api.endpoint = "api/" + api.version 
        + "/login";
}
#pragma endregion


#pragma region Functions
Token TokenManager::requestToken()
{
    Token token{};
    json usrObj;

    usrObj["username"] = user.username;
    usrObj["password"] = user.password;

    cout<<"Sending request for token"<<endl;
    auto url = api.url + api.endpoint;
    cout<<url<<endl;
    auto r = cpr::Post(cpr::Url{url},
            cpr::Body{usrObj.dump()},
       cpr::Header{{"Content-Type", "application/json"}});

    json res = json::parse(r.text);
    token.accessToken = res["token"];
    token.tokenType = res["token_type"];

    cout<<"status code "<<r.status_code<<endl;

    return token;
}
#pragma endregion

}
