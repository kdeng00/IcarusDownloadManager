#ifndef ICARUSACTION_H_
#define ICARUSACTION_H_

#include<string>
#include<vector>

#include"Flags.h"

namespace Models
{
	struct IcarusAction
	{
		std::string action;
		std::vector<Flags> flags;
	};
}

#endif
