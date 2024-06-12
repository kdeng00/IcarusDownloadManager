use std::default::Default;

use crate::models;

pub struct Download {
    pub api: models::api::API,
}

impl Default for Download {
    fn default() -> Self {
        Download {
            api: models::api::API::default(),
        }
    }
}

#[derive(Debug)]
pub enum MyError {
    _Request(reqwest::Error),
    Other(String),
}

impl Download {
    pub async fn download_song(
        &mut self,
        token: &models::token::Token,
        song: &models::song::Song,
    ) -> Result<String, MyError> {
        self.api.endpoint = String::from("song/data/download");
        let url = self.retrieve_url(&song);
        let access_token = token.bearer_token();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            http::header::HeaderValue::from_str(&access_token.clone()).unwrap(),
        );

        let client = reqwest::Client::builder().build().unwrap();
        let response = client
            .get(&url)
            .header(reqwest::header::AUTHORIZATION, &access_token)
            .send()
            .await
            .unwrap();

        match response.status() {
            reqwest::StatusCode::OK => {
                let data = response.text();
                match data.await {
                    Ok(e) => {
                        return Ok(e);
                    }
                    Err(er) => {
                        println!("Error {:?}", er);
                    }
                }
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                println!("Need to grab a new token");
            }
            other => {
                panic!("Uh oh! Something unexpected happened: {:?}", other);
            }
        }

        return Err(MyError::Other(String::from("Error downloading")));
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
