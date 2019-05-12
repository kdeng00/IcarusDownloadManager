#ifndef CONVERSIONS_H_
#define CONVERSIONS_H_

#include<memory>
#include<string>

#include<QString>

namespace Utilities
{
	class Conversions
	{
	public:
		Conversions();
		Conversions(QString);
		Conversions(QString*);

		void initializeValues();

		template <typename T>
		void printValue(T);

		std::string convertQStringToString();
		std::string convertQStringToString(QString*);
	private:
		std::unique_ptr<QString> qStrVal;
	};
}

#endif
