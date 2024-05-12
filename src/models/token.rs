use std::default::Default;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub expiration: Option<i32>,
}

impl Default for Token {
    fn default() -> Self {
        Token {
            access_token: None,
            token_type: None,
            expiration: None,
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
            None => {
            }
        }

        return token;
    }
}
