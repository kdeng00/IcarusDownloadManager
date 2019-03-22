#ifndef DOWNLOAD_H_
#define DOWNLOAD_H_

#include<iostream>
#include<string>

namespace Syncers
{
	class Download
	{
	public:
		Download();
		Download(std::string);

		void downloadSong(int);
	private:
		std::string downloadFilePath;
	};
}

#endif
