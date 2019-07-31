#ifndef DELETE_H_
#define DELETE_H_

#include"Models/API.h"
#include"Models/Song.h"
#include"Models/Token.h"

#include"SyncerBase.h"

namespace Syncers
{
    class Delete : SyncerBase
    {
    public:
        Delete(Models::API);

        void deleteSong(const Models::Token, Models::Song);
    private:
        std::string retrieveUrl(Models::Song);
    };
}

#endif
