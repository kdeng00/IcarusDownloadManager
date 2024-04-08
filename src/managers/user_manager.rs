use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct UserManager {
    user: models::User,
    ica_action: models::IcarusAction,
}

impl UserManager {
    // TODO: Implement
    pub fn retrieve_user(&self) -> models::User {
        return models::User {
            username: String::from(""),
            password: String::from(""),
        };
    }

    // TODO: Implement
    fn parse_user_from_actions(&self) {
    }
}
