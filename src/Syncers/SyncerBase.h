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
	};
}

#endif
