#include "Models/Song.h"

#include "nlohmann/json.hpp"

using std::string;

namespace Models
{
    string Song::toMetadataJson()
    {
        nlohmann::json s;
        s["title"] = this->title;

        return s.dump();
    }
}