#ifndef UPLOAD_H_
#define UPLOAD_H_

#include <filesystem>
#include <string>
#include <vector>

#include <nlohmann/json.hpp>

#include "Managers/CommitManager.h"
#include "Managers/FileManager.h"
#include "Models/API.h"
#include "Models/Song.h"
#include "Models/Token.h"
#include "Models/UploadForm.h"

namespace fs = std::filesystem;


namespace Syncers
{

class Upload
{
public:
    Upload() = default;
    Upload(Models::API api, Models::Token token) : m_token(token), api(api)
    {
        this->api.endpoint = "song/data";
    }

    Models::Song uploadSong(Models::Song&);
    void uploadSongsFromDirectory(const std::string&, const bool, bool);
    void uploadSongWithMetadata(Managers::CommitManager::Album&, Models::Song&, Models::CoverArt&);
private:
    Managers::FileManager fMgr;
    Models::API api;
    Models::Song song;
    Models::Token m_token;

    std::vector<Models::Song> retrieveAllSongsFromDirectory(const std::string&,
        bool);

    std::string retrieveUrl();

    Models::Song retrieveSongPath(fs::directory_entry&);

    void printSongDetails();
    void printSongDetails(std::vector<Models::Song>&);
    void printJsonData(const nlohmann::json&);
};

}

#endif
