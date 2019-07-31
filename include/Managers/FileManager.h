#ifndef FILEMANAGER_H_
#define FILEMANAGER_H_

#include<string>

namespace Managers
{
    class FileManager
    {
    public:
        FileManager();
        FileManager(std::string);

        void saveFile(std::string);
        void modifyFilePath(std::string);

        char* retrieveFileBuffer() const;
        int retrieveFileBufferLength() const;
    private:
        void readFile();

        std::string filePath;
        char* fileBuffer;
        bool fileRead;
        int fileBufferLength;
    };
}

#endif
