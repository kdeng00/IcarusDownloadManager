#include"Managers/CommitManager.h"

#include <iostream>
#include <fstream>
#include <sstream>
#include <filesystem>

#include "nlohmann/json.hpp"

#include "Models/API.h"
#include "Models/Song.h"
#include "Models/Token.h"
#include "Parsers/APIParser.h"
#include "Syncers/Delete.h"
#include "Syncers/Download.h"
#include "Syncers/RetrieveRecords.h"
#include "Syncers/Upload.h"

#include "Managers/TokenManager.h"
#include "Managers/UserManager.h"

using std::cout;
using std::endl;
using std::map;
using std::string;

using Managers::TokenManager;
using Managers::UserManager;
using Models::API;
using Models::Song;
using Models::Token;
using Parsers::APIParser;
using Models::IcarusAction;
using Syncers::Delete;
using Syncers::Download;
using Syncers::RetrieveRecords;
using Syncers::Upload;

namespace filesystem = std::filesystem;

namespace Managers
{

#pragma region Constructors
CommitManager::CommitManager(IcarusAction& icaAct) : icaAction(std::move(icaAct))
{ }
#pragma endregion


#pragma region Functions
void CommitManager::commitAction()
{
    auto action = icaAction.action;
    cout<<"Commiting "<<action<<" action"<<endl;

    switch (mapActions()[action])
    {
        case ActionValues::deleteAct:
            deleteSong();
            break;
        case ActionValues::downloadAct:
            downloadSong();
            break;
        case ActionValues::retrieveAct:
            retrieveObjects();
            break;
        case ActionValues::uploadAct:
            uploadSong();
            break;
        case ActionValues::UPLOAD_SONG_WITH_METADATA:
            uploadSongWithMetadata();
            break;
        default:
            break;
    }
}






std::map<std::string, CommitManager::ActionValues> 
    CommitManager::mapActions() noexcept
{
    const std::map<std::string, ActionValues> actions{
        {"delete", ActionValues::deleteAct},
        {"download", ActionValues::downloadAct},
        {"retrieve", ActionValues::retrieveAct},
        {"upload", ActionValues::uploadAct},
        {"upload-meta", ActionValues::UPLOAD_SONG_WITH_METADATA}
    };

    return actions;
}




Token CommitManager::parseToken(API api)
{
    cout<<"fetching token\n";
    UserManager usrMgr{icaAction};
    auto user = usrMgr.retrieveUser();

    TokenManager tk{user, api};
    
    return tk.requestToken();
}

void CommitManager::deleteSong()
{
    APIParser apiPrs{icaAction};
    auto api = apiPrs.retrieveAPI();

    auto token = parseToken(api);

    Song song{};

    for (auto arg : icaAction.flags)
    {
        auto flag = arg.flag;
        auto value = arg.value;

        if (flag.compare("-D") == 0)
        {
            song.id = atoi(value.c_str());
        }
    }

    Delete del{api};
    cout<<"Deleting song..."<<endl;
    del.deleteSong(token, song);
}
void CommitManager::downloadSong()
{
    cout<<"Starting downloading process..."<<endl;

    APIParser apiPrs{icaAction};
    auto api = apiPrs.retrieveAPI();

    auto token = parseToken(api);

    Song song{};

    for (auto arg : icaAction.flags)
    {
        auto flag = arg.flag;
        auto value = arg.value;

        if (flag.compare("-d") == 0)
        {
            song.songPath.assign(arg.value);
        }
        if (flag.compare("-b") == 0)
        {
            song.id = atoi(value.c_str());
        }
    }

    Download dnld{api};
    cout<<"downloading song"<<endl;
    dnld.downloadSong(token, song);
}
void CommitManager::retrieveObjects()
{
    cout<<"Starting retrieve process..."<<endl;
    
    APIParser apiPrs{icaAction};
    auto api = apiPrs.retrieveAPI();

    auto token = parseToken(api);
    RetrieveTypes retrieveType;

    for (auto arg : icaAction.flags)
    {
        auto flag = arg.flag;
        auto value = arg.value;

        if (flag.compare("-rt") == 0)
        {
            if (value.compare("songs") == 0)
            {
                retrieveType = RetrieveTypes::songs;
                break;
            }
        }
    }

    RetrieveRecords songs{api, token};
    songs.retrieve(retrieveType);

}

void CommitManager::uploadSong()
{
    auto uploadSingleSong = true;
    auto recursiveDirectory = false;
    const auto noConfirm = checkForNoConfirm();
    string songDirectory;
    APIParser apiPrs{icaAction};
    auto api = apiPrs.retrieveAPI();

    auto token = parseToken(api);

    Song song;

    for (auto& arg : icaAction.flags)
    {
        auto flag = arg.flag;
        auto value = arg.value;

        if (flag.compare("-s") == 0)
        {
            song.songPath.assign(arg.value);
        }
        else if (flag.compare("-sd") == 0)
        {
            songDirectory = value;
            uploadSingleSong = false;
        } 
        else if (flag.compare("-sr") == 0)
        {
            songDirectory = value;
            uploadSingleSong = false;
            recursiveDirectory = true;
        }
    }

    Upload upld{api, token};
    if (uploadSingleSong)
    {
        cout<<"Uploading song..."<<endl;
        upld.uploadSong(song);
    }
    else
    {
        cout<<"Uploading songs from " << songDirectory << endl;
        upld.uploadSongsFromDirectory(songDirectory, noConfirm, recursiveDirectory);
    }
}



void CommitManager::uploadSongWithMetadata()
{
    cout<<"Uploading single song with metadata\n\n";

    // Either the set of "-s", "-m", "-ca", "-t" flags or "-smca" must exist with values
    // in order to be valid but not both
    const auto songPath = this->icaAction.retrieveFlagValue("-s");
    const auto metadataPath = this->icaAction.retrieveFlagValue("-m");
    const auto coverPath = this->icaAction.retrieveFlagValue("-ca");
    const auto trackID = this->icaAction.retrieveFlagValue("-t");
    const auto singleTarget = !songPath.empty() && !metadataPath.empty() && 
        !coverPath.empty() && !trackID.empty() ? true : false;

    const auto uni = this->icaAction.retrieveFlagValue("-smca");
    const auto multiTarget = !uni.empty() ? true : false;

    if (singleTarget && multiTarget)
    {
        cout<<"Cannot upload from source and directory\n";
        return;
    }

    cout<<"Song path: "<<songPath<<"\n";
    cout<<"TrackID: "<<trackID<<"\n";
    cout<<"Metadata path: "<<metadataPath<<"\n";
    cout<<"Cover Art path: "<<coverPath<<"\n";

    if (singleTarget)
    {
        singTargetUpload(songPath, trackID, metadataPath, coverPath);
    }
    else if (multiTarget)
    {
        multiTargetUpload(uni);
    }
}


void CommitManager::singTargetUpload(const std::string &songPath, const std::string &trackID, 
    const std::string &metaPath, const std::string &coverPath)
{
    APIParser apiPrs(icaAction);
    auto api = apiPrs.retrieveAPI();
    const auto token = parseToken(api);

    auto album = retrieveMetadata(metaPath);
    album.printInfo();

    Song song;
    song.track = 1;
    song.disc = 1;

    cout<<"TrackID: "<<trackID<<"\n";

    parseDiscAndTrack<Song, std::string>(song, trackID);

    auto c = [](const Song &songA, const Song &songB) { return songA.track == songB.track && songA.disc == songB.disc; };
    auto sng = Utilities::Checks::itemIterInContainer<Song, std::vector<Song>>(album.songs, song, c);

    if (sng == album.songs.end())
    {
        cout<<"Not found with disc "<<song.disc<<" track "<<song.track<<"\n";
        std::exit(-1);
    }

    song = *sng;
    const auto p = filesystem::path(songPath);
    song.directory = p.parent_path().string();
    song.generate_filename_from_track();

    Models::CoverArt cover;
    cover.title = song.title;
    cover.path = coverPath;

    Upload up(api, token);
    up.uploadSongWithMetadata(album, song, cover);
}

void CommitManager::multiTargetUpload(const std::string &sourcePath)
{
    APIParser apiPrs(icaAction);
    auto api = apiPrs.retrieveAPI();
    const auto token = parseToken(api);

    if (!fs::is_directory(sourcePath))
    {
        cout<<sourcePath<<" is not a directory\n";
        std::exit(-1);
    }

    std::vector<Song> songs;
    Models::CoverArt cover;
    string metadataPath;

    for (auto &p: fs::directory_iterator(sourcePath))
    {
        const auto &pp = p.path();
        const auto stem = pp.stem();
        const auto file = pp.filename();
        const auto extension = pp.extension();

        cout<<"Stem "<<stem<<" Extension "<<extension<<"\n";

        if (extension.compare(".mp3") == 0)
        {
            Song song;
            song.songPath = pp.string();

            initializeDiscAndTrack<Song, std::string>(song);

            songs.emplace_back(std::move(song));
        }
        else if (extension.compare(".jpg") == 0 || extension.compare(".png") == 0)
        {
            cover.path.assign(pp.string());
        }
        else if (extension.compare(".json") == 0)
        {
            metadataPath.assign(pp.string());
        }
    }

    auto album = retrieveMetadata(metadataPath);
    songs.clear();
    songs.assign(album.songs.begin(), album.songs.end());

    Upload up(api, token);

    for (auto &song : songs)
    {
        up.uploadSongWithMetadata(album, song, cover);
    }
}

#pragma region private
CommitManager::Album CommitManager::retrieveMetadata(const std::string_view path)
{
    CommitManager::Album album;
    const auto fileContent = retrieveFileContent(path);
    cout<<"Parsing...\n";
    auto serialized = nlohmann::json::parse(fileContent);
    cout<<"Parsed\n";

    album.album = serialized["album"].get<std::string>();
    album.albumArtist = serialized["album_artist"].get<std::string>();
    album.genre = serialized["genre"].get<std::string>();
    album.year = serialized["year"].get<int>();
    album.trackCount = serialized["track_count"].get<int>();
    album.discCount = serialized["disc_count"].get<int>();
    album.songs.reserve(album.trackCount);

    for (auto &j : serialized["tracks"])
    {
        Song song;
        song.title = j["title"].get<std::string>();
        song.track = j["track"].get<int>();
        song.disc = j["disc"].get<int>();
        song.artist = j["artist"].get<std::string>();
        song.album = album.album;
        song.year = album.year;
        song.genre = album.genre;
        song.generate_filename_from_track();
        const auto p = fs::path(path);
        song.directory = p.parent_path().string();

        album.songs.push_back(song);
    }

    return album;
}

string CommitManager::retrieveFileContent(const std::string_view path)
{
    string path_str(path);
    string value;

    std::stringstream buffer;
    std::fstream file(path_str, std::ios::in);
    buffer<<file.rdbuf();
    file.close();

    value.assign(buffer.str());

    return value;
}
#pragma endregion

void CommitManager::Album::printInfo()
{
    std::cout<<"Album: "<<this->album<<"\n";
    std::cout<<"Album Artist: "<<this->albumArtist<<"\n";
    std::cout<<"Genre: "<<this->genre<<"\n";
    std::cout<<"Year: "<<this->year<<"\n";
    std::cout<<"Track count: "<<this->trackCount<<"\n";
    std::cout<<"Disc count: "<<this->discCount<<"\n";
    std::cout<<"\n";
}

#pragma region Functions

}
