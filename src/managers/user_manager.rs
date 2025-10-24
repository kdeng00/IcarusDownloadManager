use std::default::Default;

use crate::models::{self};

#[derive(Debug, Default)]
pub struct UserManager {
    pub user: icarus_models::user::User,
    pub ica_action: models::icarus_action::IcarusAction,
}

impl UserManager {
    pub fn retrieve_user(&self) -> icarus_models::user::User {
        self.user.clone()
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
