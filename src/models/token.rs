use std::default::Default;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    #[serde(alias = "user_id")]
    pub user_id: i32,
    #[serde(alias = "username")]
    pub username: Option<String>,
    #[serde(alias = "token")]
    pub access_token: Option<String>,
    #[serde(alias = "token_type")]
    pub token_type: Option<String>,
    #[serde(alias = "expiration")]
    pub expiration: Option<i32>,
    #[serde(alias = "message")]
    pub message: Option<String>,
}

impl Default for Token {
    fn default() -> Self {
        Token {
            user_id: -1,
            username: None,
            access_token: None,
            token_type: None,
            expiration: None,
            message: None,
        }
    }
}

impl Token {
    pub fn bearer_token(&self) -> String {
        let mut token: String = String::from("Bearer ");

        match &self.access_token {
            Some(tok) => {
                token += tok;
            }
            None => {}
        }

        return token;
    }
}
