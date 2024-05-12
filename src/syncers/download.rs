use std::default::Default;

use crate::models;

pub struct Download{
    pub api: models::api::API,
}

impl Default for Download {
    fn default() -> Self {
        Download {
            api: models::api::API::default(),
        }
    }
}

impl Download {
    pub async fn download_song(&self, token: &models::token::Token, song: &models::song::Song) {
        let url = self.retrieve_url(&song);
        let access_token = token.bearer_token();
        let client = reqwest::Client::new();
        let response = client.get(&url)
            .header(reqwest::header::AUTHORIZATION, &access_token)
            .send()
            .await
            .unwrap();

        match response.status() {
            reqwest::StatusCode::OK => {
                // on success, parse our JSON to an APIResponse
                /*
                let s = response.json::<Vec<Track>>().await;
                match s {
                    Ok(parsed) => {
                        println!("\nSuccess!");
                    }
                    Err(_) => println!("Hm, the response didn't match the shape we expected."),
                };
                */
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                println!("Need to grab a new token");
            }
            other => {
                panic!("Uh oh! Something unexpected happened: {:?}", other);
            }
        }
    }

    fn retrieve_url(&self, song: &models::song::Song) -> String {
        // let mut url: String = String::new();
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