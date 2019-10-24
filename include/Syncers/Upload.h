#ifndef UPLOAD_H_
#define UPLOAD_H_

#include<filesystem>
#include<string>
#include<vector>

#include<nlohmann/json.hpp>

#include"Managers/FileManager.h"
#include"Models/API.h"
#include"Models/Song.h"
#include"Models/Token.h"
#include"Models/UploadForm.h"

namespace fs = std::filesystem;


namespace Syncers
{
    class Upload
    {
        public:
            Upload();
            Upload(Models::API);

            Models::Song uploadSong(const Models::Token&, Models::Song&);
            void uploadSongsFromDirectory(const Models::Token&, const std::string&, bool);
        private:
            Managers::FileManager fMgr;
            Models::API api;
            Models::Song song;

            std::vector<Models::Song> retrieveAllSongsFromDirectory(const std::string&,
                bool);

            std::string retrieveUrl();

            Models::Song retrieveSongPath(fs::directory_entry&);

            void printSongDetails();
            void printSongDetails(std::vector<Models::Song>&);
            void printJsonData(nlohmann::json);
    };
}

#endif
