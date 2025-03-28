use std::default::Default;
use std::io::Error;

use crate::models;

pub struct RetrieveRecords {
    pub api: models::api::API,
}

impl Default for RetrieveRecords {
    fn default() -> Self {
        RetrieveRecords {
            api: models::api::API::default(),
        }
    }
}

impl RetrieveRecords {
    pub async fn get_all_songs(
        &mut self,
        token: &icarus_models::token::AccessToken,
    ) -> Result<Vec<icarus_models::song::Song>, Error> {
        self.api.endpoint = String::from("song");
        let mut songs: Vec<icarus_models::song::Song> = Vec::new();
        let url = self.retrieve_url();
        let access_token = token.bearer_token();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            http::header::HeaderValue::from_str(&access_token.clone()).unwrap(),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static("application/json"),
        );

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
                let s = response.json::<Vec<icarus_models::song::Song>>().await;
                match s {
                    //
                    Ok(parsed) => {
                        songs = parsed;
                    }
                    Err(_) => println!("Hm, the response didn't match the shape we expected."),
                };
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                println!("Need to grab a new token");
            }
            other => {
                panic!("Uh oh! Something unexpected happened: {:?}", other);
            }
        }

        return Ok(songs);
    }

    fn retrieve_url(&self) -> String {
        let api = &self.api;
        let mut url: String = String::from(&api.url);
        url += &String::from("api/");
        url += &String::from(&api.version);
        url += &String::from("/");
        url += &String::from(&api.endpoint);
        url += &String::from("/");

        return url;
    }
}
