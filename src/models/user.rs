use serde::{Deserialize, Serialize};

mod models {

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
}
}