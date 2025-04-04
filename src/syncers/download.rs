use std::default::Default;

use crate::models;
use crate::syncers;

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
                        Ok(e) => {
                            return Ok(e);
                        }
                        Err(er) => {
                            return Err(MyError::Other(er.to_string()));
                        }
                    }
                }
                reqwest::StatusCode::UNAUTHORIZED => {
                    return Err(MyError::Other(String::from("Need to grab a new token")));
                }
                other => {
                    return Err(MyError::Other(other.to_string()));
                }
            },
            Err(er) => {
                return Err(MyError::Request(er));
            }
        }
    }
}
