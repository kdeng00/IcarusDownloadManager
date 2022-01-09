#include "Syncers/Download.h"

#include <exception>
#include <iostream>
#include <fstream>

#include <cpr/cpr.h>

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

#pragma region Constructors
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
#pragma endregion


#pragma region Functions
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
                cpr::Header{{"Content-type", "audio/mpeg"},
                            {"Authorization", auth}});
            

        int statusCode = r.status_code;

        if (statusCode == OK) {
            song.data = r.text;
            saveSong(song);
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

void Download::saveSong(Song& song)
{
    cout<<"\nSaving song to: "<<song.songPath<<endl;
    int bufferLength = song.data.length();
    const char *data = song.data.c_str();
    cout<<"buff length  "<<bufferLength<<endl;

    ofstream saveSong{song.songPath, std::ios::binary};
    saveSong.write(data, bufferLength);
    saveSong.close();
}
#pragma endregion

}
