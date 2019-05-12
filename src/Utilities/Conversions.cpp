#include"Conversions.h"

#include<iostream>

using std::string;
using std::unique_ptr;

namespace Utilities
{
	Conversions::Conversions()
	{
		initializeValues();
	}
	Conversions::Conversions(QString val)
	{
		// TODO: assign QString to unique pointer
		// The line below is causing the build to fail
		qStrVal = unique_ptr<QString>(std::copy(val)};
	}
	Conversions::Conversions(QString* val)
	{
		qStrVal = unique_ptr<QString>{std::move(val)};
	}


	void Conversions::initializeValues()
	{
	}
	template <typename T>
	void Conversions::printValue(T val)
	{
		std::cout<<"going to print value"<<std::endl;
		std::cout<<val<<std::endl;
	}

	string Conversions::convertQStringToString()
	{
		auto val = qStrVal->toUtf8().constData();

		printValue(val);

		return qStrVal->toUtf8().constData();
	}
	string Conversions::convertQStringToString(QString* val)
	{
		return "";
	}
}
