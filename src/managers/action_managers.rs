use serde::{Deserialize, Serialize};

use crate::{models, utilities};

#[derive(Debug, Deserialize, Serialize)]
pub struct ActionManager {
    pub action: String,
    pub flags: Vec<models::flags::Flags>,
    pub params: Vec<String>,
    pub param_count: i32,
}

impl Default for ActionManager {
    fn default() -> Self {
        ActionManager {
            action: String::new(),
            flags: Vec::new(),
            params: Vec::new(),
            param_count: -1,
        }
    }
}

impl ActionManager {
    pub fn retrieve_icarus_action(&self) -> models::icarus_action::IcarusAction {
        return models::icarus_action::IcarusAction {
            flags: self.flags.clone(),
            action: String::from(&self.action),
        };
    }

    fn supported_flags(&self) -> Vec<String> {
        return vec![
            String::from("-u"),
            String::from("-p"),
            String::from("-t"),
            String::from("-h"),
            String::from("-s"),
            String::from("-sd"),
            String::from("-sr"),
            String::from("-d"),
            String::from("-D"),
            String::from("-b"),
            String::from("-rt"),
            String::from("-nc"),
            String::from("-m"),
            String::from("-ca"),
            String::from("-smca"),
            String::from("-t"),
        ];
    }

    pub fn initialize(&mut self) {
        self.validate_flags();
        self.validate_action();
        self.action = self.action.to_lowercase();
    }

    pub fn set_params(&mut self, args: &Vec<String>) {
        self.params = args.clone();
        self.param_count = self.params.len() as i32;
    }

    fn validate_flags(&mut self) {
        println!("Validating flags");

        let flag_vals = self.parsed_flags();

        let mut i = 0;
        println!("Flag count: {}", flag_vals.len());

        while i < flag_vals.len() {
            let flag = &flag_vals[i];
            println!("Index: {} | Value: {}", i, flag);

            let mut flg = models::flags::Flags::default();

            if self.is_valid_flag(flag) && self.does_flag_have_value(flag) {
                println!("Flag has value");
                flg.flag = String::from(flag);
                flg.value = String::from(&flag_vals[i + 1]);

                i = i + 1;
            } else if self.is_valid_flag(flag) {
                println!("Flag does not have a value");
                flg.flag = String::from(flag);
            } else {
                println!("Flag {} is not valid", flag);
                utilities::checks::exit_program(-1);
            }

            self.flags.push(flg);
            println!("");
            i += 1;
        }
    }

    fn validate_action(&mut self) {
        if self.params.len() >= 2 {
            let act = &self.params[1];
            self.action = String::from(act);
        }
    }

    fn is_valid_flag(&self, flag: &String) -> bool {
        let flags = self.supported_flags();
        let mut found: bool = false;

        for flg in &flags {
            if flg == flag {
                found = true;
                break;
            }
        }

        return found;
    }

    fn does_flag_have_value(&self, flag: &String) -> bool {
        let flags_tmp = self.parsed_flags();
        let mut i_found: i32 = -1;

        for i in 0..flags_tmp.len() {
            let flg = &flags_tmp[i];
            if flg == flag {
                i_found = i as i32;
                break;
            }
        }

        if i_found >= 0 {
            if (i_found + 1) < flags_tmp.len().try_into().unwrap() {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn _print_action(&self) {
        if self.action.len() == 0 {
            println!("Action is empty");
        } else {
            println!("Action is {}", self.action);
        }
    }

    fn _print_flags(&self) {
        println!("Printing flags...");
        for flag in &self.flags {
            println!("Flag {}", flag.flag);
            println!("Value {}", flag.value);
        }
    }

    fn parsed_flags(&self) -> Vec<String> {
        let mut parsed: Vec<String> = Vec::new();

        for i in 2..self.params.len() {
            let flag = String::from(&self.params[i]);
            parsed.push(flag);
        }

        return parsed;
    }
}
