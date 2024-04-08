use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Token {
    pub access_token: String,
    pub token_type: String,
    pub expiration: i32,
}
