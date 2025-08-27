use std::default::Default;

use http::HeaderValue;
use reqwest;

use crate::models;
use crate::syncers;

#[derive(Default)]
pub struct Upload {
    pub api: models::api::Api,
}

mod response {
    pub mod queue_song {
        #[derive(Debug, serde::Deserialize)]
        pub struct Response {
            pub message: String,
            pub data: Vec<uuid::Uuid>
        }
    }
}

impl Upload {
    pub async fn upload_song_with_metadata(
        &mut self,
        token: &icarus_models::token::AccessToken,
        song: &icarus_models::song::Song,
        cover: &icarus_models::coverart::CoverArt,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.api.endpoint = String::from("song/data/upload/with/data");
        let url = syncers::common::retrieve_url(&self.api, false, &uuid::Uuid::nil());
        let access_token = token.bearer_token();

        if url.is_empty() {
            println!("Url is empty");
        }

        println!("Url: {url}");
        println!("Token: {access_token}");
        println!("Path: {:?}", song.song_path());

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&access_token.clone()).unwrap(),
        );
        headers.insert(reqwest::header::ACCEPT, HeaderValue::from_static("*/*"));

        let form = self.init_form(song, cover);
        let client = reqwest::Client::builder().build().unwrap();
        match client
            .post(url)
            .headers(headers)
            .multipart(form)
            .send()
            .await
        {
            Ok(r) => Ok(r),
            Err(err) => Err(err),
        }
    }

    pub async fn queue_song(&self, token: &icarus_models::token::AccessToken, song: &icarus_models::song::Song) -> Result<uuid::Uuid, reqwest::Error> {
        let songpath = song.song_path().unwrap_or_default();

        let mut song_filename = String::from("audio");
        song_filename += icarus_models::constants::file_extensions::audio::DEFAULTMUSICEXTENSION;

        let form = reqwest::multipart::Form::new()
            .part(
                "file",
                reqwest::multipart::Part::bytes(std::fs::read(songpath).unwrap())
                    .file_name(song_filename),
            );

        let endpoint = String::from("api/v2/song/queue");
        let url = format!("{}/{endpoint}", self.api.url);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&token.token.clone()).unwrap(),
        );
        let client = reqwest::Client::builder().build().unwrap();
        match client.post(url).headers(headers).multipart(form).send().await {
            Ok(response) => match response.json::<response::queue_song::Response>().await {
                Ok(resp) => {
                    Ok(resp.data[0])
                }
                Err(err) => {
                    Err(err)
                }
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    fn init_form(
        &self,
        song: &icarus_models::song::Song,
        cover: &icarus_models::coverart::CoverArt,
    ) -> reqwest::multipart::Form {
        let songpath = song.song_path().unwrap_or_default();
        let coverpath = cover.path.clone();
        println!("Cover path: {coverpath:?}");
        let song_detail = song.to_metadata_json(true).unwrap_or_default();

        println!("\n{song_detail}\n");

        let mut song_filename = String::from("audio");
        song_filename += icarus_models::constants::file_extensions::audio::DEFAULTMUSICEXTENSION;
        let mut cover_filename = String::from("cover");
        cover_filename += icarus_models::constants::file_extensions::image::JPGEXTENSION;

        reqwest::multipart::Form::new()
            .part(
                "file",
                reqwest::multipart::Part::bytes(std::fs::read(songpath).unwrap())
                    .file_name(song_filename),
            )
            .part(
                "cover",
                reqwest::multipart::Part::bytes(std::fs::read(coverpath).unwrap())
                    .file_name(cover_filename),
            )
            .text("metadata", song_detail)
    }

    pub fn set_api(&mut self, host: &str) {
        let api = models::api::Api {
            url: host.to_owned(),
            version: String::from("v1"),
            endpoint: String::new(),
        };
        self.api = api;
    }
}
