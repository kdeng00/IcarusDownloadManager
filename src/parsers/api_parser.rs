
use crate::models;

pub struct APIParser {
    pub api: models::api::API,
    pub ica_act: models::icarus_action::IcarusAction,
}

impl APIParser {
    pub fn retrieve_api(&self) -> models::api::API {
        return self.api;
    }

    pub fn parse_api(&self) {
        let flags = self.ica_act.flags;
        println!("Parsing api");

        for (i, elem) in flags {
            let arg = elem.flag;
            let value = elem.value;

            if arg == "-h" {
                if value.chars().nth((value.len() - 1)) == '/' {
                    self.api.url = value;
                } else {
                    self.api.url = value + "/";
                }
                break;
            }
        }

        self.api.version = "v1";
    }
}