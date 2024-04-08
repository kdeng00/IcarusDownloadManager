use serde::{Deserialize, Serialize};

use crate::models;

// mod managers {

#[derive(Debug, Deserialize, Serialize)]
pub struct UserManager {
    user: models::user::User,
    ica_action: models::icarus_action::IcarusAction,
}

impl UserManager {
    // TODO: Implement
    pub fn retrieve_user(&self) -> models::user::User {
        return models::user::User {
            username: String::from(""),
            password: String::from(""),
        };
    }

    // TODO: Implement
    fn parse_user_from_actions(&self) {
    }
}
// }
