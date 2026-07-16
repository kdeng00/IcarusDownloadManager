use std::default::Default;

use reqwest;

use crate::models;

#[derive(Clone, Debug, Default)]
pub struct Delete {
    pub api: models::api::Api,
}

mod response {
    pub mod delete_song {
        #[derive(Debug, serde::Deserialize)]
        pub struct SongAndCoverArt {
            pub song: simodels::song::Song,
            pub coverart: simodels::coverart::CoverArt,
        }

        #[derive(Debug, serde::Deserialize)]
        pub struct Response {
            pub message: String,
            pub data: Vec<SongAndCoverArt>,
        }
    }
}

impl Delete {
    pub async fn delete_song(
        &mut self,
        token: &simodels::token::AccessToken,
        song: &simodels::song::Song,
    ) -> Result<(simodels::song::Song, simodels::coverart::CoverArt), std::io::Error> {
        self.api.endpoint = "api/v2/song".to_owned();
        let url = format!("{}{}/{}", self.api.url, self.api.endpoint, song.id);
        println!("Url: {url:?}");

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

                match response.json::<response::delete_song::Response>().await {
                    Ok(resp) => {
                        println!("Response message: {:?}", resp.message);
                        let data = &resp.data[0];
                        Ok((data.song.clone(), data.coverart.clone()))
                    }
                    Err(er) => Err(std::io::Error::other(er.to_string())),
                }
            }
            other => Err(std::io::Error::other(other.to_string())),
        }
    }
}
