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

int main(int argc, char** argv)
{
    if (argc < 2)
    {
        cout<<"No actions provided"<<endl;
        return 1;
    }

    ActionManager actMgr{argv};
    auto chosenAction = actMgr.retrieveIcarusAction();

    CommitManager commitMgr{chosenAction};
    commitMgr.commitAction();

    return 0;
}
