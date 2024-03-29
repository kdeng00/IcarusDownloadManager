#include <iostream>
#include <filesystem>
#include <exception>

#include "cpr/cpr.h"

#include "Syncers/Upload.h"
#include "Utilities/Conversions.h"

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
#pragma endregion


#pragma region Functions
Song Upload::uploadSong(Song& song)
{
    try
    {
        auto url = retrieveUrl();

        cout<<"url "<<url<<endl;
        string auth{this->m_token.tokenType};
        auth.append(" " + this->m_token.accessToken);
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

void Upload::uploadSongsFromDirectory(const std::string& directory, 
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
            Utilities::Conversions::toLowerChar(answer);

            if (answer == 'y' || answer == 'Y') 
            {
                confirmUpload = true;
                break;
            } 
        }

        cout << "uploading songs\n";
        for (auto& song: songs)
        {
            song = uploadSong(song);
        }
    }
    catch (exception& e)
    {
        cout<<e.what()<<endl;
    }
}


void Upload::uploadSongWithMetadata(Managers::CommitManager::Album &album, Models::Song& song, Models::CoverArt &cover)
{
    this->api.endpoint.assign("song/data/upload/with/data");

    try
    {
        auto url = retrieveUrl();

        cout << "url " << url << "\n";
        string auth(this->m_token.tokenType);
        auth.append(" " + this->m_token.accessToken);

        nlohmann::json s;
        s["title"] = song.title;
        s["album"] = album.album;
        s["album_artist"] = album.albumArtist;
        s["artist"] = song.artist;
        s["year"] = album.year;
        s["genre"] = album.genre;
        s["disc"] = song.disc;
        s["track"] = song.track;
        s["disc_count"] = album.discCount;
        s["track_count"] = album.trackCount;

        const auto meta = s.dump();

        cout<<"\n\nMeta:\n"<<meta<<"\n";
        cout << "Filepath: " << song.song_path() << "\n";

        auto multipart = cpr::Multipart{{"cover", cpr::File{cover.path}},
            {"metadata", meta},
            {"file", cpr::File{song.song_path()}}};

        auto r = cpr::Post(cpr::Url{url}, multipart,
            cpr::Header{{"authorization", auth}}
        );

        cout << "status code: " << r.status_code<< std::endl;
        cout << r.text << endl;
    }
    catch (exception &e)
    {
        auto msg = e.what();
        cout<<"Error: "<<msg<<"\n";
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

    cout<<endl<<endl;
}
#pragma endregion
#pragma endregion

}
