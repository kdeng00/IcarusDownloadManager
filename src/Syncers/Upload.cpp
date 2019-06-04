#include<iostream>
#include<exception>
#include<string>

#include<cpr/cpr.h>
#include<nlohmann/json.hpp>

#include"Upload.h"

using std::cout;
using std::endl;
using std::exception;
using std::string;

using json = nlohmann::json;

using Managers::FileManager;
using Models::UploadForm;

using namespace cpr;

namespace Syncers
{
	Upload::Upload() { }
	Upload::Upload(string filePath)
	{
		this->songPath = filePath;
		this->fMgr = FileManager(songPath);
	}
	Upload::Upload(UploadForm formData)
	{
		this->url = formData.url;
		this->songPath = formData.filePath;
	}


	void Upload::uploadSong()
	{
		try
		{
			auto r = cpr::Post(cpr::Url{url},
                   		cpr::Multipart{{"key", "small value"},
                                {"file", cpr::File{songPath}}});
			cout << r.status_code<< std::endl;

			cout<<"Success"<<endl;
		}
		catch(std::exception& e)
		{
			cout<<e.what()<<endl;
		}
		cout<<"Finished"<<endl;

	}
	void Upload::uploadSong(const Models::Token token, const std::string songPath)
	{
		try
		{
			string auth{token.tokenType};
			auth.append(" " + token.accessToken);
			auto r = cpr::Post(cpr::Url{""},
                   		cpr::Multipart{{"key", "small value"},
                                {"file", cpr::File{songPath}}},
		   		cpr::Header{{"authorization", auth}}
				);

			cout << r.status_code<< std::endl;
		}
		catch (exception e)
		{
			auto msg = e.what();
			cout<<msg<<endl;
		}
		cout<<"Finished"<<endl;
	}

	void Upload::configureSongDemo()
	{
		int id = 0;
		string title = "What of it?";
		string artist = "Kuoth";
		string album = "I";
		string genre = "Untitled";
		int year = 2019;
		int duration = 260;

		this->song = Models::Song{};
		this->song.id = id;
		this->song.title = title;
		this->song.artist = artist;
		this->song.album = album;
		this->song.genre = genre;
		this->song.year = year;
		this->song.duration = duration;
		this->song.songData = fMgr.retrieveFileBuffer();
		cout<<*song.songData<<endl;
	}
	void Upload::printSongDetails()
	{
		cout<<"Song details: "<<endl;
		cout<<"Id: "<<song.id<<endl;
		cout<<"Title: "<<song.title<<endl;
		cout<<"Artist: "<<song.artist<<endl;
		cout<<"Album: "<<song.album<<endl;
		cout<<"Genre: "<<song.genre<<endl;
		cout<<"Year: "<<song.year<<endl;
		cout<<"Duration: "<<song.duration<<endl;
	}
	void Upload::printJsonData(json obj)
	{
		cout<<endl<<endl<<"JSon data: "<<endl;
		cout<<"id: "<<obj["id"]<<endl;
		cout<<"title: "<<obj["title"]<<endl;
		cout<<"artist: "<<obj["artist"]<<endl;
		cout<<"album: "<<obj["album"]<<endl;
		cout<<"genre: "<<obj["genre"]<<endl;
		cout<<"year: "<<obj["year"]<<endl;
		cout<<"duration: "<<obj["duration"]<<endl;
		cout<<"song_data: "<<obj["song_data"]<<endl;

		cout<<endl<<endl;;
	}

	json Upload::serializeObject()
	{
		json jObj{};
		jObj["id"] = song.id;
		jObj["title"] = song.title;
		jObj["artist"] = song.artist;
		jObj["album"] = song.album;
		jObj["genre"] = song.genre;
		jObj["year"] = song.year;
		jObj["duration"] = song.duration;
		jObj["song_data"] = *song.songData;

		return jObj;
	}
}
