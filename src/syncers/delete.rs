use std::default::Default;

use reqwest;

use crate::models;

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
    pub async fn delete_song(
        &mut self,
        token: &icarus_models::token::AccessToken,
        song: &models::song::Song,
    ) -> Result<models::song::Song, std::io::Error> {
        self.api.endpoint = "song/data/delete".to_owned();
        let url = self.retrieve_url(&song);
        let client = reqwest::Client::builder().build().unwrap();
        let access_token = token.bearer_token();
        let response = client
            .delete(&url)
            .header(reqwest::header::AUTHORIZATION, access_token)
            .send()
            .await
            .unwrap();
        let mut sng = models::song::Song::default();

        match response.status() {
            reqwest::StatusCode::OK => {
                println!("Success!");
                let s = response.json::<models::song::Song>().await;
                match s {
                    //
                    Ok(parsed) => {
                        sng = parsed;
                    }
                    Err(er) => {
                        println!("Error {:?}", er);
                    }
                };
            }
            other => {
                panic!("Issue occurred: {:?}", other);
            }
        }

        return Ok(sng);
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
