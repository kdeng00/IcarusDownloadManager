#include<iostream>
#include<filesystem>
#include<exception>

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
        catch (exception e)
        {
            auto msg = e.what();
            cout<<msg<<endl;
        }

        return song;
    }

    void Upload::uploadSongsFromDirectory(const Models::Token& token, 
        const std::string& directory, 
        bool recursive = false)
    {
        try
        {
            auto songs = retrieveAllSongsFromDirectory(directory, recursive);
            for (auto& song: songs)
            {
                song = uploadSong(token, song);
            }
            printSongDetails(songs);

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
