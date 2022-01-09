#include "Models/Song.h"

#include "nlohmann/json.hpp"

using std::string;

namespace Models
{
    string Song::toMetadataJson()
    {
        nlohmann::json s;
        s["title"] = this->title;
        s["artist"] = this->artist;
        s["album"] = this->album;
        s["genre"] = this->genre;
        s["year"] = this->year;
        s["track"] = this->track;
        s["disc"] = this->disc;

        return s.dump();
    }
}