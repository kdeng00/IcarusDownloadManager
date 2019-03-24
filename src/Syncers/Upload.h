#ifndef UPLOAD_H_
#define UPLOAD_H_

#include<string>

#include<nlohmann/json.hpp>

#include"Managers/FileManager.h"
#include"Models/Song.h"
#include"Models/UploadForm.h"


namespace Syncers
{
	class Upload
	{
		public:
			Upload();
			Upload(std::string);
			Upload(Models::UploadForm);

			void uploadSong();

		private:
			Managers::FileManager fMgr;
			Models::Song song;
			std::string apiUrl{"http://192.168.1.3"};
			std::string apiEndPoint{"/api/song/data"};
			std::string songPath;
			std::string url;
			int port{9349};

			void configureSongDemo();
			void printSongDetails();
			void printJsonData(nlohmann::json);

			nlohmann::json serializeObject();
	};
}

#endif
