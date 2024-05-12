use std::default::Default;

use reqwest;
use reqwest::Response;

use crate::models;

pub struct Upload {
    pub api: models::api::API,
}

impl Default for Upload {
    fn default() -> Self {
        Upload {
            api: models::api::API::default(),
        }
    }
}

impl Upload {
    pub async fn upload_song(&self, token: &models::token::Token, song: &models::song::Song) {
        let url = self.retrieve_url(&song);
        let client = reqwest::Client::new();
        let access_token = token.bearer_token();
        let response = client.post(&url)
            .header(reqwest::header::AUTHORIZATION, access_token)
            .send()
            .await
            .unwrap();
    
        match response.status() {
            reqwest::StatusCode::OK => {
                println!("Success!");
            }
            other => {
                panic!("Issue occurred: {:?}", other);
            }
        }
    }


    fn retrieve_url(&self, song: &models::song::Song) -> String {
        let mut url: String = String::new();

        return url;
    }
}