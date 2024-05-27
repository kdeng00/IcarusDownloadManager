use std::default::Default;

use crate::models;

pub struct TokenManager {
    pub user: models::user::User,
    pub api: models::api::API,
}

impl Default for TokenManager {
    fn default() -> Self {
        let mut token = TokenManager {
            user: models::user::User::default(),
            api: models::api::API::default(),
        };

        token.init();

        return token;
    }
}

impl TokenManager {
    pub async fn request_token(&self) -> Result<models::token::Token, std::io::Error> {
        println!("Sending request for a token");

        // let endpoint = self.construct_endpoint();
        let mut url = self.retrieve_url();
        // url += &endpoint;

        println!("URL: {}", url);

        let mut token = models::token::Token::default();

        let client = reqwest::Client::new();
        let response = client.post(&url).json(&self.user).send().await.unwrap();

        match response.status() {
            reqwest::StatusCode::OK => {
                // on success, parse our JSON to an APIResponse
                let s = response.json::<models::token::Token>().await;
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
        let mut api = &mut self.api;
        api.version = String::from("v1");
        api.endpoint = String::from(format!("api/{}/login", api.version));
    }

    pub fn retrieve_url(&self) -> String {
        let api = &self.api;
        let mut url = String::from(&api.url);
        // url += &String::from("api/");
        // url += &String::from(&api.version);
        // url += &String::from("/");
        url += &String::from(&api.endpoint);
        url += &String::from("/");

        return url;
    }

    // NOTE: This can get deleted. Redundant
    fn construct_endpoint(&self) -> String {
        let mut endpoint: String = String::from("api/");
        endpoint += &self.api.version;
        endpoint += "/login";

        return endpoint;
    }
}
