use std::default::Default;
use std::io::Error;

use crate::models;

#[derive(Default)]
pub struct RetrieveRecords {
    pub api: models::api::Api,
}

mod response {
    pub mod get_all_songs {
        #[derive(Debug, serde::Deserialize)]
        pub struct Response {
            pub message: String,
            pub data: Vec<icarus_models::song::Song>,
        }
    }
}

impl RetrieveRecords {
    pub async fn get_all_songs(
        &mut self,
        token: &icarus_models::token::AccessToken,
    ) -> Result<Vec<icarus_models::song::Song>, Error> {
        self.api.endpoint = String::from("api/v2/song/all");
        let url = format!("{}{}", self.api.url, self.api.endpoint);
        let access_token = token.bearer_token();

        println!("url: {url:?}");

        let client = reqwest::Client::builder().build().unwrap();
        let response = client
            .get(&url)
            .header(reqwest::header::AUTHORIZATION, access_token)
            .send()
            .await
            .unwrap();

        match response.status() {
            reqwest::StatusCode::OK => {
                // on success, parse our JSON to an API Response
                match response.json::<response::get_all_songs::Response>().await {
                    Ok(parsed) => {
                        println!("Response message: {:?}", parsed.message);
                        Ok(parsed.data)
                    }
                    Err(err) => Err(std::io::Error::other(err.to_string())),
                }
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(std::io::Error::other("Need to grab a new token"))
            }
            other => Err(std::io::Error::other(other.to_string())),
        }
    }
}
