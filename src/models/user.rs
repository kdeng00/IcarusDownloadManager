use std::default::Default;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

impl Default for User {
    fn default() -> Self {
        User {
            username: String::new(),
            password: String::new(),
        }
    }
}
