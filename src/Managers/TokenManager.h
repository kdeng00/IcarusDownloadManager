#ifndef TOKENMANAGER_H_
#define TOKENMANAGER_H_

#include"Models/Token.h"
#include"Models/User.h"

namespace Managers
{
	class TokenManager
	{
	public:
		TokenManager(const Models::User);

		Models::Token requestToken();
	private:
		Models::User user;
	};
}

#endif
