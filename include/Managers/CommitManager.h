#ifndef COMMITMANAGER_H_
#define COMMITMANAGER_H_

#include<map>
#include<iostream>
#include<string>
#include<string_view>

#include"Models/API.h"
#include"Models/IcarusAction.h"
#include"Models/Song.h"
#include"Models/Token.h"
#include"Utilities/Checks.h"

namespace Managers
{

class CommitManager
{
public:
    CommitManager(Models::IcarusAction&);

    void commitAction();


    enum class RetrieveTypes
    {
        songs
    };
    
        // Used for parsing songs from the metadata file
    class Album
    {
    public:
        Album() = default;

        void printInfo();


        std::string album;
        std::string albumArtist;
        std::string genre;
        int year;
        int trackCount;
        int discCount;
        std::vector<Models::Song> songs;
    };

private:
    enum class ActionValues;
    
    std::map<std::string, ActionValues> mapActions() noexcept;

    void deleteSong();
    void downloadSong();
    void retrieveObjects();
    void uploadSong();
    // Uploads a single song. The song is constructed from a metadata file that contains
    // information about the album the song is from. Also, the cover art of the song must
    // be present.
    //
    // Expects
    // * Song - mp3 file path
    // * TrackID - track number to chose from when retrieving metadata. "1" and "1:1" are similar
    // * Metadata - Source file containing metadata of the song
    // * Cover art - path to image cover art
    void uploadSongWithMetadata();

    // Expects the song path, trackID, metadata file path, and cover path
    void singTargetUpload(const std::string &songPath, const std::string &trackID, 
        const std::string &metaPath, const std::string &coverPath);
    // Expects the source directory that contains songs, a metadata file, and cover image
    // Disc and Track is retrieved from the filename if the filename conforms to a standard.
    // If not, then the disc and track will default to 1
    //
    // Standards
    // * track01.mp3 - Disc 1, Track 1
    // * track05d02.mp3 - Disc 2, Track 5
    void multiTargetUpload(const std::string &sourcePath);

    // Standards
    // * track01.mp3 - Disc 1, Track 1
    // * track05d02.mp3 - Disc 2, Track 5
    template<typename Song, typename Str>
    void initializeDiscAndTrack(Song &song)
    {
        auto disc = 1;
        auto track = 1;
        // If 1 go with first standard, if 2 go with the second, if 0 then will default to 1 for disc and track
        auto mode = 0;
        const Str &songPath = song.songPath;

        auto trd = song.songPath.find("trackd");
        auto tr = song.songPath.find("track");

        if (tr != Str::npos)
        {
            mode = 1;
        }

        if (trd != Str::npos)
        {
            mode = 2;
        }

        auto dl = [](char c, char t){ return c == t; };
        auto d = Utilities::Checks::itemIterInContainer<char, Str>(songPath, 'd', dl);
        auto k = Utilities::Checks::itemIterInContainer<char, Str>(songPath, 'k', dl);
        auto dot = Utilities::Checks::itemIterInContainer<char, Str>(songPath, '.', dl);

        switch(mode)
        {
        case 1:
        {
            if (k != songPath.end() && dot != songPath.end())
            {
                auto tStr = std::string(++k, dot);
                std::cout << "TStr: " << tStr<<"\n";

                if (Utilities::Checks::isNumber(tStr))
                {
                    track = std::atoi(tStr.c_str());
                }
            }
            break;
        }
        case 2:
        {
            if (k != songPath.end() && dot != songPath.end() && d != songPath.end())
            {
                auto tStr = std::string(++k, d);
                auto dStr = std::string(++d, dot);
                std::cout<<"DStr: "<<dStr<<" TStr: " << tStr<<"\n";

                if (Utilities::Checks::isNumber(tStr))
                {
                    track = std::atoi(tStr.c_str());
                }
                else if (Utilities::Checks::isNumber(dStr))
                {
                    disc = std::atoi(dStr.c_str());
                }
            }
            break;
        }
        }


        song.disc = disc;
        song.track = track;
    }

    template<typename Song, typename Str>
    void parseDiscAndTrack(Song &song, const Str &trackID)
    {
        auto sep = [](char c, char t) { return c == t; };
        auto separator = Utilities::Checks::itemIterInContainer<char, Str>(trackID, ':', sep);

        if (separator != trackID.end())
        {
            auto dStr = Str(trackID.begin(), separator);
            auto tStr = Str(++separator, trackID.end());

            song.disc = std::atoi(dStr.c_str());
            song.track = std::atoi(tStr.c_str());
        }
        else
        {
            auto isNumber = Utilities::Checks::isNumber(trackID);
            if (isNumber)
            {
                song.track = std::atoi(trackID.c_str());
            }
        }
    }


    // Checks for the no confirm flag. Used when uploading songs from a directory
    bool checkForNoConfirm()
    {
        for (const auto &arg: this->icaAction.flags)
        {
            if (arg.flag.compare("-nc") == 0)
            {
                return true;
            }
        }

        return false;
    }


    Album retrieveMetadata(const std::string_view path);
    std::string retrieveFileContent(const std::string_view path);

    enum class ActionValues
    {
        deleteAct,
        downloadAct,
        retrieveAct,
        uploadAct,
        UPLOAD_SONG_WITH_METADATA // Uploads the song with metadata, including cover art
    };


    Models::IcarusAction icaAction;

};

}

#endif
