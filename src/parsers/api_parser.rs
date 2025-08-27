use crate::models;

#[derive(Clone, Debug)]
pub struct APIParser {
    pub apis: Vec<models::api::Api>,
    pub ica_act: models::icarus_action::IcarusAction,
}

pub const API_VERSION: &str = "v2";

pub enum APIType {
    Main,
    Auth,
}

impl APIParser {
    pub fn retrieve_api(&self, api_type: APIType) -> models::api::Api {
        match api_type {
            APIType::Main => self.apis[0].clone(),
            APIType::Auth => self.apis[1].clone(),
        }
    }

    pub fn parse_api(&mut self, api_type: APIType) {
        let flags = self.ica_act.flags.clone();
        println!("Parsing api");

        for elem in flags {
            let arg = elem.flag;
            let value = elem.value;

            match api_type {
                APIType::Main => {
                    if arg == "-h" {
                        if value.chars().nth(value.len() - 1) == Some('/') {
                            self.apis[0].url = value;
                        } else {
                            self.apis[0].url = value + "/";
                        }
                        break;
                    }
                }
                APIType::Auth => {
                    if arg == "-ha" {
                        if value.chars().nth(value.len() - 1) == Some('/') {
                            self.apis[1].url = value;
                        } else {
                            self.apis[1].url = value + "/";
                        }
                        break;
                    }
                }
            }
        }

        // for api in self.apis {
        // }
        // self.api.version = String::from(API_VERSION);
    }
}
