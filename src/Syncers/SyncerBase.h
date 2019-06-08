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
	};
}

#endif
