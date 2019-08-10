#ifndef UPLOAD_H_
#define UPLOAD_H_

#include<string>

#include<nlohmann/json.hpp>

#include"Managers/FileManager.h"
#include"Models/API.h"
#include"Models/Song.h"
#include"Models/Token.h"
#include"Models/UploadForm.h"


namespace Syncers
{
    class Upload
    {
        public:
            Upload();
            Upload(Models::API);

            void uploadSong(const Models::Token, Models::Song);
        private:
            Managers::FileManager fMgr;
            Models::API api;
            Models::Song song;

            std::string retrieveUrl();

            void printSongDetails();
            void printJsonData(nlohmann::json);
    };
}

#endif
