#ifndef SONG_H_
#define SONG_H_

#include<string>

#include<nlohmann/json.hpp>

using json = nlohmann::json;

namespace Models
{
	struct Song
	{
		int id;
		std::string title;
		std::string artist;
		std::string album;
		std::string genre;
		int year;
		int duration;
		char* songData;
	};
} 

#endif
