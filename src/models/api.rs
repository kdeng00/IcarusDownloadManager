use std::default::Default;

use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct API {
    pub url: String,
    pub endpoint: String,
    pub version: String,
}

impl Default for API {
    fn default() -> Self {
        API {
            url: String::from(""),
            endpoint: String::from(""),
            version: String::from(""),
        }
    }
}