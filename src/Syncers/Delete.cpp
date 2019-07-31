#include"Syncers/Delete.h"

#include<exception>
#include<iostream>

#include<cpr/cpr.h>

using std::cout;
using std::endl;
using std::exception;
using std::string;

using Models::API;
using Models::Song;
using Models::Token;

namespace Syncers
{
    #pragma
    Delete::Delete(API api)
    {
        this->api = api;
        this->api.endpoint = "song/data";
    }
    #pragma Constructors


    #pragma
    void Delete::deleteSong(const Token token, Song song)
    {
        try
        {
            auto url = retrieveUrl(song);
            string auth{token.tokenType};
            auth.append(" " + token.accessToken);
            auto r = cpr::Delete(cpr::Url(url),
                cpr::Header{{"authorization", auth}});

            auto statusCode = r.status_code;

            cout<<"Status code "<<statusCode<<endl;
        }
        catch (exception e)
        {
            auto msg = e.what();
            cout<<msg<<endl;
        }
        cout<<"Finished"<<endl;
    }

    string Delete::retrieveUrl(Song song)
    {
        string url{api.url + "api/" + api.version + "/" +
            api.endpoint + "/"};

        url.append(std::to_string(song.id));
        cout<<"url "<<url<<endl;

        return url;
    }
    #pragma Functions
}
