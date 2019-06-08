#include"Download.h"

#include<exception>
#include<iostream>
#include<fstream>

#include<cpr/cpr.h>

using std::cout;
using std::endl;
using std::exception;
using std::ofstream;
using std::string;

using Models::API;
using Models::Song;
using Models::Token;

namespace Syncers
{
	#pragma
	Download::Download() { }
	Download::Download(API api)
	{
		this->api = api;
		this->api.endpoint = "song/data";
	}
	Download::Download(string filePath)
	{
		downloadFilePath = filePath;
	}
	#pragma Constructors


	#pragma
	void Download::downloadSong(const int id)
	{
		string urlRoot = "http://192.168.1.5";
		int port = 9349;
		string endpoint = "api/song/data/" + std::to_string(id);
		string url = urlRoot + ":" + std::to_string(port) + "/" +
					 endpoint;
		auto r = cpr::Get(cpr::Url{url});
		const char* newBuff = r.text.c_str();
		int bufferLength = r.text.size();

		ofstream saveSong{downloadFilePath, ofstream::binary};
		saveSong.write(newBuff, bufferLength);

		cout<<"HTTP status code: "<<r.status_code<<endl;
		cout<<"Header info: "<<r.header["content-type"]<<endl;
		cout<<"Header info: "<<endl;
		cout<<r.header["content-type"]<<endl;
		cout<<r.header["content-disposition"]<<endl;
	}
	void Download::downloadSong(const Token token, Song song)
	{
		try
		{
			string url = retrieveUrl(song);
			song.songPath.append("track.mp3");
			cout<<"song path "<<song.songPath<<endl;
			string auth{token.tokenType};
			auth.append(" " + token.accessToken);
			auto r = cpr::Get(cpr::Url(url), 
				cpr::Header{{"authorization", auth}});

			int statusCode = r.status_code;

			if (statusCode == OK)
			{
				song.data = r.text;
				saveSong(&song);
			}


			cout<<"finsihed with status code "<<statusCode<<endl;
		}
		catch (exception e)
		{
			auto msg = e.what();
			cout<<msg<<endl;
		}
	}

	string Download::retrieveUrl(Song song)
	{
		string url{api.url + "api/" + api.version + "/" + 
			api.endpoint + "/"};

		url.append(std::to_string(song.id));
		cout<<"url "<<url<<endl;

		return url;
	}

	void Download::saveSong(Song *song)
	{
		cout<<"\nSaving song to: "<<song->songPath<<endl;
		int bufferLength = song->data.length();
		const char *data = song->data.c_str();
		cout<<"buff length  "<<bufferLength<<endl;

		ofstream saveSong{song->songPath, std::ios::binary};
		saveSong.write(data, bufferLength);
		saveSong.close();
	}
	#pragma Functions
}
