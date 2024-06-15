use std::default::Default;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct API {
    pub url: String,
    pub endpoint: String,
    pub version: String,
}

impl Default for API {
    fn default() -> Self {
        API {
            url: String::new(),
            endpoint: String::new(),
            version: String::new(),
        }
    }
}
