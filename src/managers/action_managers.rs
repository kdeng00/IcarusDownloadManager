use serde::{Deserialize, Serialize};

use crate::models;

// mod managers {

#[derive(Debug, Deserialize, Serialize)]
pub struct ActionManager {
    pub action: String,
    pub flags: Vec<models::flags::Flags>,
    pub params: Vec<String>,
    pub param_count: i32,
}

impl ActionManager {
    // TODO: Implement
    pub fn retrieve_icarus_action(&self) -> models::icarus_action::IcarusAction {
        return models::icarus_action::IcarusAction {
            action: String::from(""),
            flags: Vec::new(),
        };
    }

    // TODO: Implement
    fn supported_flags(&self) -> Vec<String> {
        return Vec::new();
    }

    // TODO: Implement
    fn supported_actions(&self) {}
    // TODO: Implement
    fn initialize(&self) {}
    // TODO: Implement
    fn validate_flags(&self) {}
    // TODO: Implement
    fn is_valid_flag(&self, flag: &String) -> bool {
        return false;
    }
    // TODO: Implement
    fn does_flag_have_value(&self, flag: &String) -> bool {
        return false;
    }
    // TODO: Implement
    fn print_action(&self) {}
    // TODO: Implement
    fn print_flags(&self) {}
    // TODO: Implement
    fn parsed_flags(&self) -> Vec<String> {
        return Vec::new();
    }
}
// }
