#ifndef SONG_H_
#define SONG_H_

#include <string>
#include <iostream>
#include <sstream>


namespace Models
{

class Song
{
public:
    Song() = default;

    void printInfo()
    {
        std::cout<<"Title: "<<this->title<<"\n";
        std::cout<<"\n";
    }

    std::string song_path() noexcept
    {
        std::stringstream buffer;

        buffer << this->directory;

        const auto count = this->directory.size();

        if (this->directory.at(count - 1) != '/' || this->directory.at(count - 1) != '\\')
        {
            buffer << "/";
        }

        buffer << this->filename;

        return buffer.str();
    }

    int generate_filename_from_track()
    {
        auto result = 0;
        std::stringstream buffer;
        buffer << "track";

        // NOTE: Multiple discs in one directory is not being addressed
        if (this->track < 10)
        {
            buffer << "0";
        }

        buffer << this->track << ".mp3";


        this->filename.assign(buffer.str());

        return result;
    }

    std::string toMetadataJson();


    int id;
    std::string title;
    std::string artist;
    std::string album;
    std::string genre;
    int year;
    int duration;
    int track;
    int disc;
    std::string data;
    [[deprecated("Use song_path() function instead")]]
    std::string songPath;
    std::string filename;
    std::string directory;
};

class CoverArt
{
public:
    int id;
    std::string title;
    std::string path;
};

} 

#endif
