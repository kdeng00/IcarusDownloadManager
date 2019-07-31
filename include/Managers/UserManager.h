#ifndef USERMANAGER_H_
#define USERMANAGER_H_

#include<iostream>

#include"Models/IcarusAction.h"
#include"Models/User.h"

namespace Managers
{
    class UserManager
    {
    public:
        UserManager(Models::User);
        UserManager(const Models::IcarusAction);

        Models::User retrieveUser() const;
    private:
        void parseUserFromActions();

        Models::User user;
        Models::IcarusAction icaAction;
    };
}

#endif
