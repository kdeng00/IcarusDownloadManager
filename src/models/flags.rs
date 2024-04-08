use std::default::Default;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Flags {
    pub flag: String,
    pub value: String,
}

impl Default for Flags {
    fn default() -> Self {
        Flags {
            flag: String::new(),
            value: String::new(),
        }
    }
}
