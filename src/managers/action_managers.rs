use serde::{Deserialize, Serialize};

mod managers {

#[derive(Debug, Deserialize, Serialize)]
pub struct ActionManager {
    action: String,
    flags: Vec<models::Flags>,
    params: Vec<String>,
    param_count: i32,
}

impl ActionManager {
    // TODO: Implement
    pub fn retrieve_icarus_action(&self) -> models::IcarusAction {
        return models::IcarusAction {
            action: String::from(""),
            flags: Vec::new(),
        };
    }

    // TODO: Implement
    fn supported_flags(&self) -> Vec<String> {
        return Vec::new();
    }

    // TODO: Implement
    fn supported_actions(&self) {
    }
    // TODO: Implement
    fn initialize(&self) {
    }
    // TODO: Implement
    fn validate_flags(&self) {
    }
    // TODO: Implement
    fn is_valid_flag(&self, flag: &String) -> bool{
        return false;
    }
    // TODO: Implement
    fn does_flag_have_value(&self, flag: &String) -> bool {
        return false;
    }
    // TODO: Implement
    fn print_action(&self) {
    }
    // TODO: Implement
    fn print_flags(&self) {
    }
    // TODO: Implement
    fn parsed_flags(&self) -> Vec<String> {
        return Vec::new();
    }
}
}
