use std::default::Default;
use std::io::Error;

use crate::models;
use crate::syncers;

#[derive(Default)]
pub struct RetrieveRecords {
    pub api: models::api::API,
}

/*
impl Default for RetrieveRecords {
    fn default() -> Self {
        RetrieveRecords {
            api: models::api::API::default(),
        }
    }
}
*/

impl RetrieveRecords {
    pub async fn get_all_songs(
        &mut self,
        token: &icarus_models::token::AccessToken,
    ) -> Result<Vec<icarus_models::song::Song>, Error> {
        self.api.endpoint = String::from("song");
        let url = syncers::common::retrieve_url(&self.api, false, &uuid::Uuid::nil());
        let access_token = token.bearer_token();

        let client = reqwest::Client::builder().build().unwrap();
        let response = client
            .get(&url)
            .header(reqwest::header::AUTHORIZATION, access_token)
            .send()
            .await
            .unwrap();

        match response.status() {
            reqwest::StatusCode::OK => {
                // on success, parse our JSON to an APIResponse
                match response.json::<Vec<icarus_models::song::Song>>().await {
                    Ok(parsed) => Ok(parsed),
                    Err(err) => {
                        Err(std::io::Error::other(
                            err.to_string(),
                        ))
                    }
                }
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(std::io::Error::other(
                    "Need to grab a new token",
                ))
            }
            other => {
                Err(std::io::Error::other(
                    other.to_string(),
                ))
            }
        }
    }
}
