#include<iostream>
#include<filesystem>
#include<exception>

#include<cpr/cpr.h>
#include<nlohmann/json.hpp>

#include"Syncers/Upload.h"

using std::cout;
using std::cin;
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

#pragma region Constructors
Upload::Upload() { }
Upload::Upload(API api) : api(api)
{
    this->api.endpoint = "song/data";
}
#pragma endregion


#pragma region Functions
Song Upload::uploadSong(const Models::Token& token, Song& song)
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

        cout << "status code: " << r.status_code<< std::endl;
        cout << r.text << endl;
        auto result = nlohmann::json::parse(r.text);
        cout<<"Finished"<<endl;
        song.id = result["id"].get<int>();
        song.title = result["title"].get<std::string>();
        song.artist = result["artist"].get<std::string>();
        song.album = result["album"].get<std::string>();
        song.genre = result["genre"].get<std::string>();
        song.year = result["year"].get<int>();
        song.duration = result["duration"].get<int>();
        song.track = result["track"].get<int>();

        return song;
    }
    catch (exception& e)
    {
        auto msg = e.what();
        cout<<msg<<endl;
    }

    return song;
}

void Upload::uploadSongsFromDirectory(const Models::Token& token, 
    const std::string& directory, 
    const bool noConfirm, bool recursive = false)
{
    try
    {
        auto songs = retrieveAllSongsFromDirectory(directory, recursive);
        auto confirmUpload = true;

        while (!noConfirm) 
        {
            auto answer = 'a';
            cout << "are you sure you want to upload " << songs.size() << " songs? [y/n]";
            cin >> answer;

            if (answer == 'y' || answer == 'Y') 
            {
                confirmUpload = true;
                break;
            } 
            else if (answer == 'n' || answer == 'N') 
            {
                confirmUpload = false;
                break;
            }
        }

        if (!confirmUpload) 
        {
            cout << "exiting...\n";
            std::exit(-1);
        }

        cout << "uploading songs\n";
        for (auto& song: songs)
        {
            song = uploadSong(token, song);
        }
    }
    catch (exception& e)
    {
        cout<<e.what()<<endl;
    }
}


std::vector<Song> Upload::retrieveAllSongsFromDirectory(const std::string& directory,
    bool recursive)
{
    std::vector<Song> allSongs;
    if (recursive)
    {
        for (auto p: fs::recursive_directory_iterator(directory))
        {
            auto song = retrieveSongPath(p);
            if (!song.songPath.empty())
                allSongs.push_back(song);
        }
    }
    else
    {
        for (auto p: fs::directory_iterator(directory))
        {
            auto song = retrieveSongPath(p);
            if (!song.songPath.empty())
                allSongs.push_back(song);
        }
    }

    return allSongs;
}


string Upload::retrieveUrl()
{
    const string url{api.url + "api/" + api.version + "/" +
        api.endpoint};
        
    return url;
}


Song Upload::retrieveSongPath(fs::directory_entry& dirEntry)
{
    constexpr auto mp3Ext = ".mp3";
    Song song;
    if (fs::is_regular_file(dirEntry.path()))
    {
        const auto ext = dirEntry.path().extension().string();
        if (ext.compare(mp3Ext) == 0)
        {
            cout << "found mp3 file" << endl;
            song.songPath = dirEntry.path().string();
        }
    }

    return song;
}


#pragma region Testing
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
void Upload::printSongDetails(std::vector<Song>& songs)
{
    for (auto& song: songs)
    {
        cout<<"Song details: "<<endl;
        cout<<"Id: "<<song.id<<endl;
        cout<<"Title: "<<song.title<<endl;
        cout<<"Artist: "<<song.artist<<endl;
        cout<<"Album: "<<song.album<<endl;
        cout<<"Genre: "<<song.genre<<endl;
        cout<<"Year: "<<song.year<<endl;
        cout<<"Duration: "<<song.duration<<endl;
        cout<<"Path: "<<song.songPath<<endl;
    }
}
void Upload::printJsonData(const json& obj)
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
#pragma endregion
#pragma endregion

}
