#include"Download.h"

#include<iostream>
#include<fstream>

#include<cpr/cpr.h>

using std::cout;
using std::endl;
using std::ofstream;
using std::string;

using Models::API;

namespace Syncers
{
	Download::Download() { }
	Download::Download(API api)
	{
		this->api = api;
	}
	Download::Download(string filePath)
	{
		downloadFilePath = filePath;
	}


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
}
