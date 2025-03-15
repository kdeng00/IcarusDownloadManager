use std::default::Default;

use http::HeaderMap;
use http::HeaderValue;
use reqwest;
use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use crate::models;

pub struct Upload {
    pub api: models::api::API,
}

#[derive(Debug, Deserialize, Serialize)]
struct Song {
    title: String,
    album: String,
    artist: String,
    album_artist: String,
    year: i32,
    genre: String,
    duration: i32,
    track: i32,
    track_count: i32,
    disc: i32,
    disc_count: i32,
    #[serde(skip_serializing)]
    songpath: String,
}

impl Song {
    pub fn to_metadata_json(&self) -> Result<String, serde_json::Error> {
        return serde_json::to_string_pretty(&self);
    }
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
        cover: &models::song::CoverArt,
        album: &models::song::Album,
    ) -> Result<reqwest::Response, std::io::Error> {
        self.api.endpoint = String::from("song/data/upload/with/data");
        let url = self.retrieve_url();
        let mut new_song = self.initialize_song(&song, &album);
        match song.song_path() {
            Ok(p) => {
        new_song.songpath = p;
            }
            Err(er) => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error with song path"));
            }
        }
        let access_token = token.bearer_token();

        if url.is_empty() {
            println!("Url is empty");
        }

        println!("Url: {}", url);
        println!("Token: {}", access_token);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&access_token.clone()).unwrap(),
        );
        headers.insert(reqwest::header::ACCEPT, HeaderValue::from_static("*/*"));

        let form = self.init_form(&new_song, &cover);
        let client = reqwest::Client::builder().build().unwrap();
        let response = client
            .post(url)
            .headers(headers)
            .multipart(form)
            .send()
            .await;
        let response_text = response.unwrap();

        println!("Something was sent");
        println!("{:?}", response_text);

        return Ok(response_text);
    }

    fn _initialize_form(
        &self,
        song_raw_data: Vec<u8>,
        cover_raw_data: Vec<u8>,
        song_detail: String,
    ) -> Form {
        let mut headers = HeaderMap::new();
        headers.insert(
            http::header::CONTENT_TYPE,
            http::HeaderValue::from_static("application/octet-stream"),
        );

        let file = reqwest::multipart::Part::bytes(song_raw_data).headers(headers);
        let mut headers_i = HeaderMap::new();
        headers_i.insert(
            http::header::CONTENT_TYPE,
            http::HeaderValue::from_static("image/jpeg"),
        );

        let cover = reqwest::multipart::Part::bytes(cover_raw_data).headers(headers_i);

        let mut song_filename = String::from("audio");
        song_filename += icarus_models::constants::WAV_EXTENSION;
        let mut cover_filename = String::from("cover");
        cover_filename += icarus_models::constants::JPG_EXTENSION;

        return reqwest::multipart::Form::new()
            .part("cover", cover.file_name(cover_filename))
            .text("metadata", song_detail)
            .part("file", file.file_name(song_filename));
    }

    fn init_form(&self, song: &Song, cover: &models::song::CoverArt) -> reqwest::multipart::Form {
        let songpath = song.songpath.clone();
        let coverpath = cover.path.clone().unwrap();
        let song_detail = song.to_metadata_json().unwrap();

        println!("\n{}\n", song_detail);

        let mut song_filename = String::from("audio");
        song_filename += icarus_models::constants::WAV_EXTENSION;
        let mut cover_filename = String::from("cover");
        cover_filename += icarus_models::constants::JPG_EXTENSION;

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

    fn initialize_song(&self, song: &icarus_models::song::Song, album: &models::song::Album) -> Song {
        let dur = song.duration.clone();
        println!("Duration: {}", dur);

        return Song {
            title: String::from(&song.title.clone()),
            album: album.title.clone(),
            artist: String::from(&song.artist.clone().clone()),
            album_artist: album.album_artist.clone(),
            year: album.year.clone(),
            genre: album.genre.clone(),
            // duration: f64::round(dur) as i32,
            duration: dur,
            track: (song.track.clone()),
            track_count: album.track_count.clone(),
            disc: song.disc.clone(),
            disc_count: album.disc_count.clone(),
            songpath: String::new(),
        };
    }
}
