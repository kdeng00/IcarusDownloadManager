use crate::{models, utilities};

#[derive(Debug)]
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
        models::icarus_action::IcarusAction {
            flags: self.flags.clone(),
            action: String::from(&self.action),
        }
    }

    fn supported_flags(&self) -> Vec<String> {
        vec![
            String::from("-u"),
            String::from("-p"),
            String::from("-t"),
            String::from("-h"),
            String::from("-ha"),
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
            String::from("-v"),
        ]
    }

    pub fn initialize(&mut self) {
        self.validate_flags();
        self.validate_action();
        self.action = self.action.to_lowercase();
    }

    pub fn set_params(&mut self, args: &[String]) {
        self.params = args.to_owned();
        self.param_count = self.params.len() as i32;
    }

    fn validate_flags(&mut self) {
        let flag_vals = self.parsed_flags();

        let mut i = 0;

        for flag in &flag_vals {
            let mut flg = models::flags::Flags::default();

            if self.is_valid_flag(flag) && self.does_flag_have_value(flag) {
                if self.does_flag_have_value(flag) {
                    flg.flag = String::from(flag);
                    flg.value = String::from(&flag_vals[i + 1]);

                    i += 1;
                } else {
                    flg.flag = String::from(flag);
                }
            } else {
                utilities::checks::exit_program(-1);
            }

            self.flags.push(flg);
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

        found
    }

    fn does_flag_have_value(&self, flag: &String) -> bool {
        let flags_tmp = self.parsed_flags();
        let mut i_found: i32 = -1;

        for (i, item) in flags_tmp.iter().enumerate() {
            let flg = &item;
            if *flg == flag {
                i_found = i as i32;
                break;
            }
        }

        if i_found >= 0 {
            (i_found + 1) < flags_tmp.len().try_into().unwrap()
        } else {
            false
        }
    }

    fn _print_action(&self) {
        if self.action.is_empty() {
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

        if self.param_count <= 2 {
            let flag = String::from(&self.params[1]);
            if flag == "-v" {
                parsed.push(flag);
            }
        } else {
            for i in 2..self.params.len() {
                let flag = String::from(&self.params[i]);
                parsed.push(flag);
            }
        }

        parsed
    }
}
