use std::default::Default;

use crate::models;


#[derive(Clone, Debug, Default)]
pub struct IcarusAction {
    pub action: String,
    pub flags: Vec<models::flags::Flags>,
}

impl IcarusAction {
    pub fn retrieve_flag_value(&self, flag: &String) -> String {
        let mut val: String = String::new();

        for f in self.flags.iter() {
            if f.flag == *flag {
                val = f.value.clone();
                break;
            }
        }

        val
    }

    pub fn print_action_and_flags(&self) {
        println!("Action: {}", self.action);
        println!("Flag count: {}", self.flags.len());

        for flag in self.flags.iter() {
            println!("flag {} value {}", flag.flag, flag.value);
        }
    }
}
