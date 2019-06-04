#include"TokenManager.h"

#include<iostream>

#include<cpr/cpr.h>
#include<nlohmann/json.hpp>

using std::cout;
using std::endl;

using json = nlohmann::json;

using Managers::TokenManager;
using Models::Token;
using Models::User;

namespace Managers
{
	#pragma
	TokenManager::TokenManager(const User user)
	{
		this->user = user;
	}
	#pragma Constructors


	#pragma
	Token TokenManager::requestToken()
	{
		Token token{};
		json usrObj;

		usrObj["username"] = user.username;
		usrObj["password"] = user.password;

		auto r = cpr::Post(cpr::Url{""},
				cpr::Body{usrObj.dump()},
		   cpr::Header{{"Content-Type", "application/json"}});

		json res = json::parse(r.text);
		token.accessToken = res["token"];
		token.tokenType = res["token_type"];

		return token;
	}
	#pragma Functions
}
