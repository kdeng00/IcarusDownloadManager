#ifndef RETRIEVERECORDS_H_
#define RETRIEVERECORDS_H_

#include "Managers/CommitManager.h"
#include "Models/API.h"
#include "Models/Token.h"
#include "Syncers/SyncerBase.h"

namespace Syncers
{

class RetrieveRecords: public SyncerBase
{
public:
    RetrieveRecords();
    RetrieveRecords(Models::API, Models::Token);

    void retrieve(Managers::CommitManager::RetrieveTypes);
private:
    void fetchSongs();

    Models::API api;
    Models::Token token;
};

}

#endif
