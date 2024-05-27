use std::default::Default;
use std::io::Error;

use reqwest;
use reqwest::Response;
use serde;
// use serde::Deserialize;
// use serde::Serialize;

use crate::models;

use super::syncer_base;

#[derive(Clone, Debug)]
pub struct Delete {
    pub api: models::api::API,
}

impl Default for Delete {
    fn default() -> Self {
        Delete {
            api: models::api::API::default(),
        }
    }
}

impl Delete {
    pub async fn delete_song(&self, token: &models::token::Token, song: &models::song::Song) {
        let url = self.retrieve_url(&song);
        let client = reqwest::Client::new();
        let access_token = token.bearer_token();
        let response = client
            .delete(&url)
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
        let api = &self.api;
        let mut url: String = String::from(&api.url);
        url += &String::from("api/");
        url += &String::from(&api.version);
        url += &String::from("/");
        url += &String::from(&api.endpoint);
        url += &String::from("/");
        url += &song.id.unwrap().to_string();

        return url;
    }
}
