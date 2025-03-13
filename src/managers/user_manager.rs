use std::default::Default;

use serde::{Deserialize, Serialize};

use crate::models::{self};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserManager {
    pub user: models::user::User,
    pub ica_action: models::icarus_action::IcarusAction,
}

impl Default for UserManager {
    fn default() -> Self {
        UserManager {
            user: models::user::User::default(),
            ica_action: models::icarus_action::IcarusAction::default(),
        }
    }
}

impl UserManager {
    pub fn retrieve_user(&self) -> models::user::User {
        return self.user.clone();
    }

    pub fn parse_user_from_actions(&mut self) {
        let args = &self.ica_action.flags;

        // Quit the loop when two are found
        let mut amt: i32 = 0;
        for arg in args {
            let flag = &arg.flag;

            if flag == "-u" {
                self.user.username = String::from(&arg.value);
                amt += 1;
            } else if flag == "-p" {
                self.user.password = String::from(&arg.value);
                amt += 1;
            }

            if amt == 2 {
                break;
            }
        }
    }
}
