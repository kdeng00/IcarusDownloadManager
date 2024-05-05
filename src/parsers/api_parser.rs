use crate::models;

#[derive(Clone, Debug)]
pub struct APIParser {
    pub api: models::api::API,
    pub ica_act: models::icarus_action::IcarusAction,
}

impl APIParser {
    pub fn retrieve_api(&self) -> models::api::API {
        return self.api.clone();
    }

    pub fn parse_api(&mut self) {
        let flags = self.ica_act.flags.clone();
        println!("Parsing api");

        let mut i = 0;
        // for (i, elem) in flags {
        for elem in flags {
            let arg = elem.flag;
            let value = elem.value;

            if arg == "-h" {
                if value.chars().nth((value.len() - 1)) == Some('/') {
                    self.api.url = value;
                } else {
                    self.api.url = value + "/";
                }
                break;
            }

            i += 1;
        }

        self.api.version = "v1".to_string();
    }
}
