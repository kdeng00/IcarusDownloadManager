#ifndef COMMITMANAGER_H_
#define COMMITMANAGER_H_

#include<map>
#include<string>

#include"Models/API.h"
#include"Models/Token.h"
#include"Models/IcarusAction.h"

namespace Managers
{
    class CommitManager
    {
    public:
        CommitManager(Models::IcarusAction);

        void commitAction();
    private:
        Models::Token parseToken(Models::API);

        void initializeMapActions();
        void deleteSong();
        void downloadSong();
        void uploadSong();

        enum ActionValues
        {
            deleteAct,
            downloadAct,
            retrieveAct,
            uploadAct
        };

        std::map<std::string, ActionValues> mapActions;
        Models::IcarusAction icaAction;
    };
}

#endif
