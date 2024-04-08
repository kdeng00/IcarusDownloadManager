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
    pub fn retrieve_flag_value(&self, flag: &String) -> String {
        let mut val: String = String::new();

        for f in self.flags.iter() {
            if f.flag == *flag {
                val = flag.clone();
                break;
            }
        }

        return val;
    }

    pub fn print_action_and_flags(&self) {
        println!("Action: {}", self.action);
        println!("Flag count: {}", self.flags.len());

        for flag in self.flags.iter() {
            println!("flag {} value {}", flag.flag, flag.value);
        }
    }
}
