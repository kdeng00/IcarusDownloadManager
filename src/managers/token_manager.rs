use std::default::Default;

use crate::models;

pub struct TokenManager {
    pub user: icarus_models::user::User,
    pub api: models::api::API,
}

impl Default for TokenManager {
    fn default() -> Self {
        let mut token = TokenManager {
            user: icarus_models::user::User::default(),
            api: models::api::API::default(),
        };

        token.init();

        return token;
    }
}

impl TokenManager {
    pub async fn request_token(&self) -> Result<icarus_models::token::AccessToken, std::io::Error> {
        println!("Sending request for a token");

        let url = self.retrieve_url();

        println!("URL: {}", url);

        let mut token = icarus_models::token::AccessToken {
            user_id: uuid::Uuid::nil(),
            username: String::new(),
            token: String::new(),
            token_type: String::new(),
            expiration: -1,
            message: String::new(),
        };

        let client = reqwest::Client::new();
        let response = client.post(&url).json(&self.user).send().await.unwrap();

        match response.status() {
            reqwest::StatusCode::OK => {
                // on success, parse our JSON to an APIResponse
                let s = response.json::<icarus_models::token::AccessToken>().await;
                match s {
                    //
                    Ok(parsed) => {
                        token = parsed;
                    }
                    Err(_) => println!("Hm, the response didn't match the shape we expected."),
                };
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                println!("Need to grab a new token");
            }
            other => {
                panic!("Uh oh! Something unexpected happened: {:?}", other);
            }
        }

        return Ok(token);
    }

    pub fn init(&mut self) {
        let api = &mut self.api;
        api.version = String::from("v1");
        api.endpoint = String::from(format!("api/{}/login", api.version));
    }

    pub fn retrieve_url(&self) -> String {
        let api = &self.api;
        let mut url = String::from(&api.url);
        url += &String::from(&api.endpoint);

        return url;
    }
}
