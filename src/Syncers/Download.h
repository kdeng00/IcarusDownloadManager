#ifndef DOWNLOAD_H_
#define DOWNLOAD_H_

#include<iostream>
#include<string>

#include"Models/API.h"
#include"Models/Song.h"

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
	private:
		std::string downloadFilePath;
	};
}

#endif
