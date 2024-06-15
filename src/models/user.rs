use std::default::Default;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
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

impl User {
    pub fn _to_json(&self) -> Result<String, serde_json::Error> {
        return serde_json::to_string_pretty(&self);
    }
}
