use std::default::Default;
use std::io::Error;

use crate::models;

// use super::syncer_base::Result;


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
    pub async fn get_all_songs(&self, token: &models::token::Token) 
        -> Result<Vec<models::song::Song>, Error> {

        let mut songs: Vec<models::song::Song> = Vec::new();
        let url = self.retrieve_url();
        let access_token = token.bearer_token();
        let client = reqwest::Client::new();
        let response = client.get(&url)
            .header(reqwest::header::AUTHORIZATION, access_token)
            .send()
            .await
            .unwrap();


        match response.status() {
            reqwest::StatusCode::OK => {
                // on success, parse our JSON to an APIResponse
                let s = response.json::<Vec<models::song::Song>>().await;
                match s {
                    // 
                    Ok(parsed) => {
                        songs = parsed;
                        },
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
        // let mut url: String = String::new();
        let api = &self.api;
        let mut url: String = String::from(&api.url);
        url += &String::from("api/");
        url += &String::from(&api.version);
        url += &String::from("/");
        url += &String::from(&api.endpoint);
        url += &String::from("/");
        // url += &song.id.unwrap().to_string();

        return url;
    }
}