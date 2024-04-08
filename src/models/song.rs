use std::default::Default;

use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
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
    // TODO: Implement
    pub fn print_info(&self) {}

    // TODO: Implement
    pub fn song_path(&self) -> String {
        return String::from("");
    }

    // TODO: Implement
    pub fn generate_filename_from_track() -> i32 {
        return 0;
    }

    // TODO: Implement
    pub fn to_metadata_json() -> String {
        return String::from("");
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
