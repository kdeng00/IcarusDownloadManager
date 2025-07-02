use crate::models;

#[derive(Clone, Debug)]
pub struct APIParser {
    pub api: models::api::Api,
    pub ica_act: models::icarus_action::IcarusAction,
}

impl APIParser {
    pub fn retrieve_api(&self) -> models::api::Api {
        self.api.clone()
    }

    pub fn parse_api(&mut self) {
        let flags = self.ica_act.flags.clone();
        println!("Parsing api");

        for elem in flags {
            let arg = elem.flag;
            let value = elem.value;

            if arg == "-h" {
                if value.chars().nth(value.len() - 1) == Some('/') {
                    self.api.url = value;
                } else {
                    self.api.url = value + "/";
                }
                break;
            }
        }

        self.api.version = "v1".to_string();
    }
}
