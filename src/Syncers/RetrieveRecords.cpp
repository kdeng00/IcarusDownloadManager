#include "Syncers/RetrieveRecords.h"

#include <iostream>
#include <fstream>

#include <cpr/cpr.h>

using std::cout;
using std::endl;
using std::ofstream;

using Managers::CommitManager;
using Models::API;
using Models::Token;

namespace Syncers
{

    RetrieveRecords::RetrieveRecords() { }
    RetrieveRecords::RetrieveRecords(API api, Token token) 
        : token(token), api(api) { }

    void RetrieveRecords::retrieve(CommitManager::RetrieveTypes type)
    {
        switch (type)
        {
            case CommitManager::RetrieveTypes::songs:
                fetchSongs();
                break;
            default:
                break;
        }
    }
    void RetrieveRecords::fetchSongs()
    {
        cout<<"fetching songs"<<endl;
        auto url = api.url + "api/" + api.version + "/" + "song";
        cout<<url<<endl;

        auto auth = token.tokenType;
        auth.append(" " + token.accessToken);
        auto r = cpr::Get(cpr::Url{url},
             cpr::Header{{"authorization", auth},
                 });

        ofstream writeData{};
        writeData.open("songs.json");
        writeData<<r.text;
        writeData.close();
    }
}
