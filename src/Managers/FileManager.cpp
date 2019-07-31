#include<iostream>
#include<fstream>

#include"Managers/FileManager.h"

using std::cout;
using std::endl;
using std::ifstream;
using std::ofstream;
using std::string;

namespace Managers
{
    FileManager::FileManager() {}
    FileManager::FileManager(string filePath)
    {
        this->filePath = filePath;
        readFile();
    }   


    void FileManager::saveFile(string newFilePath)
    {
        if (!fileRead)
            readFile();

        ofstream of{newFilePath, ofstream::binary};
        of.write(fileBuffer, fileBufferLength);
        of.close();
    }

    void FileManager::readFile()
    {
        ifstream is{filePath, ifstream::binary};
        if (is)
        {
            is.seekg (0, is.end);
            fileBufferLength = is.tellg();
            is.seekg (0, is.beg);

            fileBuffer = new char [fileBufferLength];

            cout<< "Reading "<<fileBufferLength<<" characters... "<<endl;;
            is.read (fileBuffer,fileBufferLength);

            if (is)
                cout<<"all characters read successfully.";
            else
                cout<<"error: only "<<is.gcount()<<" could be read";
            cout<<endl;
            is.close();
            fileRead = true;
        }
    }
    void FileManager::modifyFilePath(string filePath)
    {
        this->filePath = filePath;
    }

    char* FileManager::retrieveFileBuffer() const
    {
        return fileBuffer;
    }

    int FileManager::retrieveFileBufferLength() const { return fileBufferLength; }
}
