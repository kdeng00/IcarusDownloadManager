use std::default::Default;

use crate::models;

mod response {
    pub mod token {
        #[derive(Debug, serde::Deserialize, serde::Serialize)]
        pub struct Response {
            pub message: String,
            pub data: Vec<icarus_models::login_result::LoginResult>,
        }
    }
}

pub struct TokenManager {
    pub user: icarus_models::user::User,
    pub api: models::api::Api,
}

impl Default for TokenManager {
    fn default() -> Self {
        let mut token = TokenManager {
            user: icarus_models::user::User::default(),
            api: models::api::Api::default(),
        };
        token.init();

        token
    }
}

impl TokenManager {
    pub async fn request_token(&self) -> Result<icarus_models::token::AccessToken, std::io::Error> {
        println!("Sending request for a token");

        let url = self.retrieve_url();
        println!("URL: {url}");

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
                match response.json::<response::token::Response>().await {
                    Ok(response) => {
                        let login_result = &response.data[0];
                        token.user_id = login_result.id;
                        token.username = login_result.username.clone();
                        token.token = login_result.token.clone();
                        token.token_type = login_result.token_type.clone();
                        token.expiration = login_result.expiration;
                        token.message = response.message;
                        Ok(token)
                    }
                    Err(err) => Err(std::io::Error::other(err.to_string())),
                }
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(std::io::Error::other("Need to grab a new token"))
            }
            other => {
                panic!("Uh oh! Something unexpected happened: {other:?}");
            }
        }
    }

    pub fn init(&mut self) {
        let api = &mut self.api;
        api.version = String::from(crate::parsers::api_parser::API_VERSION);
        api.endpoint = format!("api/{}/login", api.version);
    }

    pub fn retrieve_url(&self) -> String {
        format!("{}{}", self.api.url, self.api.endpoint)
    }
}
