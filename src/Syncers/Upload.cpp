#include<iostream>
#include<exception>
#include<string>

#include<cpr/cpr.h>
#include<nlohmann/json.hpp>

#include"Syncers/Upload.h"

using std::cout;
using std::endl;
using std::exception;
using std::string;

using json = nlohmann::json;

using Managers::FileManager;
using Models::API;
using Models::Song;
using Models::UploadForm;

using namespace cpr;

namespace Syncers
{
    Upload::Upload() { }
    Upload::Upload(API api) : api(api)
    {
        this->api.endpoint = "song/data";
    }


    void Upload::uploadSong(const Models::Token token, Song song)
    {
        try
        {
            auto url = retrieveUrl();

            cout<<"url "<<url<<endl;
            string auth{token.tokenType};
            auth.append(" " + token.accessToken);
            auto r = cpr::Post(cpr::Url{url},
                        cpr::Multipart{{"key", "small value"},
                                {"file", cpr::File{song.songPath}}},
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

    string Upload::retrieveUrl()
    {
        const string url{api.url + "api/" + api.version + "/" +
            api.endpoint};
            
        return url;
    }
    #pragma
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
    #pragma Testing
    #pragma Functions
}
