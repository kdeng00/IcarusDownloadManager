use std::default::Default;

use serde::{Deserialize, Serialize};

use crate::models;


#[derive(Debug, Deserialize, Serialize)]
pub struct IcarusAction {
    pub action: String,
    pub flags: Vec<models::flags::Flags>,
}

impl Default for IcarusAction {
    fn default() -> Self {
        IcarusAction {
            action: String::new(),
            flags: Vec::new(),
        }
    }
}

impl IcarusAction {
    // TODO: Implement
    pub fn retrieve_flag_value(&self, flag: &String) -> String {
        return String::from("");
    }

    // TODO: Implement
    pub fn print_action_and_flags(&self) {}
}
