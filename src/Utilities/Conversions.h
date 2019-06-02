#ifndef CONVERSIONS_H_
#define CONVERSIONS_H_

#include<memory>
#include<string>

namespace Utilities
{
	class Conversions
	{
	public:
		Conversions();

		void initializeValues();

		template <typename T>
		void printValue(T);
	private:
	};
}

#endif
