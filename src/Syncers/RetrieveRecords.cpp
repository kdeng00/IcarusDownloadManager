#include "Syncers/RetrieveRecords.h"

#include <iostream>
#include <fstream>

#include <cpr/cpr.h>
#include <nlohmann/json.hpp>

#include "Utilities/Conversions.h"

using std::cout;
using std::endl;
using std::ofstream;

using Managers::CommitManager;
using Models::API;
using Models::Token;
using Utilities::Conversions;

namespace Syncers
{

#pragma region Constructors
RetrieveRecords::RetrieveRecords() { }
RetrieveRecords::RetrieveRecords(API api, Token token) 
    : token(token), api(api) { }
#pragma endregion

#pragma region Functions
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

    auto auth = token.tokenType;
    auth.append(" " + token.accessToken);
    auto r = cpr::Get(cpr::Url{url},
         cpr::Header{{"authorization", auth},
             });

    if (r.status_code != (int)Result::OK) {
        cout<<"something went wrong\n";
        cout<<"status code: "<<r.status_code<<endl;
        cout<<"message: "<<r.text<<endl;
        return;
    }
    auto songData = nlohmann::json::parse(r.text);

    ofstream writeData{};
    writeData.open("songs.json");
    writeData<<songData.dump(4);
    writeData.close();
}
#pragma endregion

}
