use std::default::Default;

use reqwest::blocking::multipart;
use serde::{Deserialize, Serialize};
use reqwest;
// use reqwest::blocking::
use reqwest::{Body, Client};
use reqwest::multipart::{Form, Part};
use reqwest::Response;
use tokio::fs::File;

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
    duration: f64,
    track: i32,
    track_count: i32,
    disc: i32,
    disc_count: i32,
}

impl Default for Upload {
    fn default() -> Self {
        Upload {
            api: models::api::API::default(),
        }
    }
}

impl Upload {
    pub async fn upload_song(&self, token: &models::token::Token, song: &models::song::Song) {
        let url = self.retrieve_url(&song);
        let client = reqwest::Client::new();
        let access_token = token.bearer_token();
        let response = client
            .post(&url)
            .header(reqwest::header::AUTHORIZATION, access_token)
            .send()
            .await
            .unwrap();

        match response.status() {
            reqwest::StatusCode::OK => {
                println!("Success!");
            }
            other => {
                panic!("Issue occurred: {:?}", other);
            }
        }
    }

    // TODO: Implement
    pub async fn upload_song_with_metadata(&mut self, token: &models::token::Token, song: &models::song::Song,
        cover: &models::song::CoverArt, album: &models::song::Album) {
            self.api.endpoint = String::from("song/data/upload/with/data");
            let url = self.retrieve_url(&song);
            let client = reqwest::Client::new();
            let new_song = self.initialize_song(&song, &album);
            let access_token = token.bearer_token();
        
        let song_data = song.to_data();
        let cover_data = cover.to_data();

        match song_data {
            Ok(sd) => {
            },
            Err(er) => {
                println!("Error: {:?}", er);
                std::process::exit(-1);
            }
        }

        match cover_data {
            Ok(_) => {},
            Err(er) => {
                println!("Error: {:?}", er);
                std::process::exit(-1);
            }
        }

        if url.is_empty() {
            println!("Url is empty");
        }

        /*

        let s_file = std::fs::File::open(song.song_path());
        let s_stream = tokio_util::codec::FramedRead::new(s_file, tokio_util::codec::BytesCodec::new());
        let s_file_body = Body::wrap_stream(s_stream);
        let s_some_file = multipart::Part::stream(s_file_body)
            .file_name("track.cdda.wav");

        let s_data = Part::file(song.song_path());
        let c_data = multipart::Part::stream(&cover.path);

        let mut form = Form::new();
        let meta = serde_json::to_string_pretty(&new_song);
        form = form.text("metadata", meta.unwrap());

        form = form.part("file", s_some_file)
            .part("cover", c_data);
    
        let response = client.post(url).multipart(form).send();
        let result = response.text;

        return Ok(result);
        */
    }



    pub fn set_api(&mut self, host: &String) {
        let mut api = models::api::API::default();
        api.url = host.clone();
        api.version = String::from("v1");
        self.api = api;
    }
    fn retrieve_url(&self, song: &models::song::Song) -> String {
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

    fn initialize_song(&self, song: &models::song::Song, album: &models::song::Album) -> Song {
        return Song {
            title: String::from(&song.title.clone().unwrap()),
            album: album.title.clone(),
            artist: String::from(&song.artist.clone().unwrap().clone()),
            album_artist: album.album_artist.clone(),
            year: album.year.clone(),
            genre: album.genre.clone(),
            duration: song.duration.clone().unwrap(),
            track: (song.track.clone().unwrap()),
            track_count: album.track_count.clone(),
            disc: song.disc.clone().unwrap(),
            disc_count: album.disc_count.clone(),
        };
    }
}
