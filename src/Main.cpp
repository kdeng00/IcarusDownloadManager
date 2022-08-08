#include<iostream>
#include<string>

#include"Managers/ActionManager.h"
#include"Managers/CommitManager.h"

using std::cin;
using std::cout;
using std::endl;
using std::string;

using Managers::ActionManager;
using Managers::CommitManager;

constexpr static auto IcarusDownloadManager_version()
{
    return "v0.3.0";
}

void printHelp()
{
    cout<<"icd [Action] [flag]\n\n";

    cout<<"Actions\n";
    cout<<"download\n";
    cout<<"upload\n";
    cout<<"upload-meta\n";
    cout<<"retrieve\n";
    cout<<"delete\n\n";

    cout<<"Flags\n";
    cout<<"Required for all actions\n";
    cout<<"-u username\n";
    cout<<"-p password\n";
    cout<<"-h host\n\n";

    cout<<"Required for upload\n";
    cout<<"-s path of song\n";
    cout<<"-sd directory where to search for songs to upload (Optional)\n";
    cout<<"-sr directory where to recursively search for songs to upload (Optional)\n";
    cout<<"-nc will not prompt the user when uploading from a directory\n\n";

    cout<<"Required for upload with metadata\n";
    cout<<"-s path of song\n";
    cout<<"-t track number\n";
    cout<<"-m metadata filepath\n";
    cout<<"-ca coverart filepath\n";
    cout<<"-scma directory where songs, metadata, and cover art exists and will be uploaded (Optional)\n\n";

    cout<<"Required for download\n";
    cout<<"-b song id\n";
    cout<<"-d path to download song (Optional)\n\n";

    cout<<"Required for retrieving records\n";
    cout<<"-rt retrieve type (songs is only accepted)\n\n";

    cout<<"Required for deleting a song\n";
    cout<<"-D song id\n\n";
}


int main(int argc, char** argv)
{
    if (argc < 2)
    {
        printHelp();
        return -1;
    }

    ActionManager actMgr(argv, argc);
    auto chosenAction = actMgr.retrieveIcarusAction();

    chosenAction.print_action_and_flags();

    CommitManager commitMgr(chosenAction);
    commitMgr.commitAction();

    return 0;
}
