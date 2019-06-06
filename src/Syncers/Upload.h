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
			Upload(std::string);
			Upload(Models::API);
			Upload(Models::UploadForm);

			void uploadSong();
			void uploadSong(const Models::Token, Models::Song);
		private:
			Managers::FileManager fMgr;
			Models::API api;
			Models::Song song;
			std::string apiUrl{""}; // Not being used
			std::string apiEndPoint{""}; // Not being used
			std::string songPath; // Not being used
			std::string url; // Not being used
			int port{9349}; // Not being used

			std::string retrieveUrl();

			void configureSongDemo();
			void printSongDetails();
			void printJsonData(nlohmann::json);

			nlohmann::json serializeObject(); // Not being used
	};
}

#endif
