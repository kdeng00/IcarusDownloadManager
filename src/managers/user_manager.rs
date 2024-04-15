use serde::{Deserialize, Serialize};

use crate::models;


#[derive(Debug, Deserialize, Serialize)]
pub struct UserManager {
    pub user: models::user::User,
    pub ica_action: models::icarus_action::IcarusAction,
}

impl UserManager {
    pub fn retrieve_user(&self) -> models::user::User {
        return self.user.clone();
    }

    pub fn parse_user_from_actions(&mut self) {
        let args = &self.ica_action.flags;

        for arg in args {
            let flag = &arg.flag;

            if flag == "-u" {
                self.user.username = String::from(&arg.value);
            }

            if flag == "-p" {
                self.user.password = String::from(&arg.value);
            }
        }
    }
}
