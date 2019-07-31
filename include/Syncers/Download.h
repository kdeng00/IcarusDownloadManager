#ifndef DOWNLOAD_H_
#define DOWNLOAD_H_

#include<iostream>
#include<string>

#include"Models/API.h"
#include"Models/Song.h"
#include"Models/Token.h"

#include"SyncerBase.h"

namespace Syncers
{
    class Download : SyncerBase
    {
    public:
        Download();
        Download(Models::API);
        Download(std::string);

        void downloadSong(int);
        void downloadSong(const Models::Token token, Models::Song);
    private:
        std::string retrieveUrl(Models::Song);

        std::string downloadFilePath;
        void saveSong(Models::Song*);
    };
}

#endif
