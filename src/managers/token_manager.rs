use std::default::Default;

use crate::models;

pub struct TokenManager {
    pub user: models::user::User,
    pub api: models::api::API,
}


impl TokenManager {
    pub fn request_token(&self) -> models::token::Token {
        let usr_json = self.user.to_json();
        println!("Sending request for a token");

        let endpoint = self.construct_endpoint();
        let mut url: String = String::from(&self.api.url);
        url += &endpoint;

        println!("URL: {}", url);

        return models::token::Token::default();
    }

    fn construct_endpoint(&self) -> String {
        let mut endpoint: String = String::from("api/");
        endpoint += &self.api.version;
        endpoint += "/login";

        return endpoint;
    }
}