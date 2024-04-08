use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct IcarusAction {
    pub action: String,
    pub flags: Vec<models::Flags>,
}

impl IcarusAction {
    // TODO: Implement
    pub fn retrieve_flag_value(&self, flag: &String) -> String {
        return String::from("");
    }

    // TODO: Implement
    pub fn print_action_and_flags(&self) {
    }
}
