use std::default::Default;

use http::HeaderValue;
use reqwest;

use crate::models;

pub struct Upload {
    pub api: models::api::API,
}

impl Default for Upload {
    fn default() -> Self {
        Upload {
            api: models::api::API::default(),
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
        let url = self.retrieve_url();
        let access_token = token.bearer_token();

        if url.is_empty() {
            println!("Url is empty");
        }

        println!("Url: {}", url);
        println!("Token: {}", access_token);
        println!("Path: {:?}", song.song_path());

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&access_token.clone()).unwrap(),
        );
        headers.insert(reqwest::header::ACCEPT, HeaderValue::from_static("*/*"));

        let form = self.init_form(&song, &cover);
        let client = reqwest::Client::builder().build().unwrap();
        match client
            .post(url)
            .headers(headers)
            .multipart(form)
            .send()
            .await
        {
            Ok(r) => {
                return Ok(r);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    fn init_form(
        &self,
        song: &icarus_models::song::Song,
        cover: &icarus_models::coverart::CoverArt,
    ) -> reqwest::multipart::Form {
        let songpath = match song.song_path() {
            Ok(s) => s,
            Err(_) => String::new(),
        };
        let coverpath = cover.path.clone();
        println!("Cover path: {:?}", coverpath);
        let song_detail = match song.to_metadata_json(true) {
            Ok(s) => s,
            Err(_) => String::new(),
        };

        println!("\n{}\n", song_detail);

        let mut song_filename = String::from("audio");
        song_filename += icarus_models::constants::file_extensions::audio::DEFAULTMUSICEXTENSION;
        let mut cover_filename = String::from("cover");
        cover_filename += icarus_models::constants::file_extensions::image::JPGEXTENSION;

        let form = reqwest::multipart::Form::new()
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
            .text("metadata", song_detail);

        return form;
    }

    pub fn set_api(&mut self, host: &String) {
        let mut api = models::api::API::default();
        api.url = host.clone();
        api.version = String::from("v1");
        self.api = api;
    }

    fn retrieve_url(&self) -> String {
        let api = &self.api;
        let mut buffer = api.url.clone();
        let count = buffer.len();

        if buffer.chars().nth(count - 1) != Some('/') {
            buffer += "/";
        }

        let mut url: String = String::from(&buffer);
        url += &String::from("api/");
        url += &String::from(&api.version);
        url += &String::from("/");
        url += &String::from(&api.endpoint);

        return url;
    }
}
