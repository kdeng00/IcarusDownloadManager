use std::default::Default;

use crate::models;
use crate::syncers;

#[derive(Default)]
pub struct Download {
    pub api: models::api::Api,
}

#[derive(Debug)]
pub enum MyError {
    Request(reqwest::Error),
    Other(String),
}

impl Download {
    pub async fn download_song(
        &mut self,
        token: &icarus_models::token::AccessToken,
        song: &icarus_models::song::Song,
    ) -> Result<String, MyError> {
        self.api.endpoint = String::from("song/data/download");
        let url = syncers::common::retrieve_url(&self.api, true, &song.id);
        let access_token = token.bearer_token();

        println!("Url: {url:?}");

        let client = reqwest::Client::builder().build().unwrap();

        match client
            .get(&url)
            .header(reqwest::header::AUTHORIZATION, &access_token)
            .send()
            .await
        {
            Ok(rep) => match rep.status() {
                reqwest::StatusCode::OK => {
                    let data = rep.text();
                    match data.await {
                        Ok(e) => Ok(e),
                        Err(er) => Err(MyError::Other(er.to_string())),
                    }
                }
                reqwest::StatusCode::UNAUTHORIZED => {
                    Err(MyError::Other(String::from("Need to grab a new token")))
                }
                other => Err(MyError::Other(other.to_string())),
            },
            Err(er) => Err(MyError::Request(er)),
        }
    }
}
