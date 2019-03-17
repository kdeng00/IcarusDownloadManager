IcarusDownloadManager: src/Main.cpp src/Managers/FileManager.h src/Managers/FileManager.cpp src/Syncers/Upload.h src/Syncers/Upload.cpp src/Models/Song.h 
	g++ -std=c++11 -I src/ src/Main.cpp src/Managers/FileManager.h src/Managers/FileManager.cpp src/Syncers/Upload.h src/Syncers/Upload.cpp src/Models/Song.h -o build/IcarusDownloadManager
