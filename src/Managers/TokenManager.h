#ifndef TOKENMANAGER_H_
#define TOKENMANAGER_H_

#include"Models/API.h"
#include"Models/Token.h"
#include"Models/User.h"

namespace Managers
{
	class TokenManager
	{
	public:
		TokenManager(const Models::User);
		TokenManager(const Models::User, Models::API);

		Models::Token requestToken();
	private:
		Models::API api;
		Models::User user;
	};
}

#endif
