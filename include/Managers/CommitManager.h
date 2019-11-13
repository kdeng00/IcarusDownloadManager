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
        CommitManager(Models::IcarusAction&);

        void commitAction();

        enum class RetrieveTypes
        {
            songs
        };

    private:
        enum class ActionValues;

        std::map<std::string, ActionValues> mapActions() noexcept;

        Models::Token parseToken(Models::API);

        void deleteSong();
        void downloadSong();
        void retrieveObjects();
        void uploadSong();

        enum class ActionValues
        {
            deleteAct,
            downloadAct,
            retrieveAct,
            uploadAct
        };

        Models::IcarusAction icaAction;
    };
}

#endif
