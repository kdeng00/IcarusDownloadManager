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

        Models::Token parseToken(Models::API);

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
        // * Metadata - Source file containing metadata of the song
        // * Cover art - path to image cover art
	    void uploadSingleSong();

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
