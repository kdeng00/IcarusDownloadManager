#include"APIParser.h"

using Models::API;
using Models::IcarusAction;

namespace Parsers
{
	#pragma
	APIParser::APIParser(IcarusAction icaAct)
	{
		this->icaAct = icaAct;
		api = API{};
		parseAPI();
	}
	#pragma endregion


	#pragma
	API APIParser::retrieveAPI() const
	{
		return api;
	}

	void APIParser::parseAPI()
	{
		auto flags = icaAct.flags;

		for (auto flag :  flags)
		{
			auto arg = flag.flag;
			auto value = flag.value;

			if (arg.compare("-h") == 0)
			{
				api.url = value;
			}
		}

		// TODO: For now I will hard code
		// the api version since I am only
		// on version 1
		api.version = "v1";
	}
	#pragma functions
}
