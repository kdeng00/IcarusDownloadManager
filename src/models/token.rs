use serde::{Deserialize, Serialize};

// mod models {

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub expiration: i32,
}
// }
