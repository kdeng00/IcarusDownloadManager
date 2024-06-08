use std::default::Default;
use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Song {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub year: Option<i32>,
    pub duration: Option<f64>,
    pub track: Option<i32>,
    pub disc: Option<i32>,
    pub data: Option<String>,
    // use filepath instead
    // pub song_path: String,
    pub filepath: Option<String>,
    pub directory: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Album {
    #[serde(alias = "album")]
    pub title: String,
    pub album_artist: String,
    pub genre: String,
    pub year: i32,
    pub track_count: i32,
    pub disc_count: i32,
    #[serde(alias = "tracks")]
    pub songs: Vec<Song>,
}

impl Default for Album {
    fn default() -> Self {
        Album {
            title: String::new(),
            album_artist: String::new(),
            genre: String::new(),
            year: 0,
            track_count: 0,
            disc_count: 0,
            songs: Vec::new(),
        }
    }
}

impl Default for Song {
    fn default() -> Self {
        Song {
            id: None,
            title: None,
            artist: None,
            album: None,
            genre: None,
            year: None,
            duration: None,
            track: None,
            disc: None,
            data: None,
            filepath: None,
            directory: None,
        }
    }
}

impl Song {
    pub fn print_info(&self) {
        println!("Title: {:?}", self.title);
    }

    pub fn song_path(&self) -> String {
        let directory =
            &<std::option::Option<std::string::String> as Clone>::clone(&self.directory).unwrap();

        let mut buffer: String = directory.to_string();
        let count = buffer.len();

        if buffer.chars().nth(count - 1) != Some('/') {
            buffer += "/";
        }

        let filename =
            &<std::option::Option<std::string::String> as Clone>::clone(&self.filepath).unwrap();
        buffer += filename;

        return buffer;
    }

    pub fn to_data(&self) -> Result<Vec<u8>, std::io::Error> {
        let path = self.song_path();
        println!("Converting song to data");
        println!("Path: {:?}", path);
        // let content = std::fs::read_to_string(path);
        let mut file = std::fs::File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        if buffer.len() == 0 {
            println!("Why is it empty?");
        }

        Ok(buffer)
    }
    // if 1 - wav, if 0 - mp3, anything else defaults to wav
    pub fn generate_filename_from_track(&mut self, i_type: i32) -> i32 {
        let mut filename: String = String::new();
        if self.track.unwrap() < 10 {
            filename += "0";
        }

        filename += &self.track.unwrap().to_string();

        if i_type == 0 {
            filename += ".mp3";
        } else {
            filename += ".wav";
        }

        self.filepath = Some(filename);

        return 0;
    }

    pub fn to_metadata_json(&self) -> Result<String, serde_json::Error> {
        return serde_json::to_string_pretty(&self);
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CoverArt {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub path: Option<String>,
}

impl Default for CoverArt {
    fn default() -> Self {
        CoverArt {
            id: None,
            title: None,
            path: None,
        }
    }
}

impl CoverArt {
    pub fn to_data(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut path: String = String::new();
        match &self.path {
            Some(val) => {
                path = String::from(val);
            }
            None => {
                ();
            }
        }

        let mut file = std::fs::File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}
