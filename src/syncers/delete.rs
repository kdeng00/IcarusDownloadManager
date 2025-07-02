use std::default::Default;

use reqwest;

use crate::models;
use crate::syncers;

#[derive(Clone, Debug, Default)]
pub struct Delete {
    pub api: models::api::Api,
}

impl Delete {
    pub async fn delete_song(
        &mut self,
        token: &icarus_models::token::AccessToken,
        song: &icarus_models::song::Song,
    ) -> Result<icarus_models::song::Song, std::io::Error> {
        self.api.endpoint = "song/data/delete".to_owned();
        let url = syncers::common::retrieve_url(&self.api, true, &song.id);
        let client = reqwest::Client::builder().build().unwrap();
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

                match response.json::<icarus_models::song::Song>().await {
                    Ok(sng) => Ok(sng),
                    Err(er) => Err(std::io::Error::other(er.to_string())),
                }
            }
            other => Err(std::io::Error::other(other.to_string())),
        }
    }
}
