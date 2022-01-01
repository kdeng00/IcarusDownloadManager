#ifndef SONG_H_
#define SONG_H_

#include<string>
#include<iostream>


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
        std::string songPath;
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
