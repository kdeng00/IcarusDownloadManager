use std::default::Default;
use std::io::Read;

use serde::{Deserialize, Serialize};

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
    pub songs: Vec<icarus_models::song::Song>,
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

/*

    // if 1 - wav, if 0 - mp3, anything else defaults to wav
    pub fn _generate_filename_from_track(&mut self, i_type: i32) -> i32 {
        let mut filename: String = String::new();
        if self.track.unwrap() < 10 {
            filename += "0";
        }

        filename += &self.track.unwrap().to_string();

        if i_type == 0 {
            filename += icarus_models::constants::MPTHREE_EXTENSION;
        } else {
            filename += icarus_models::constants::WAV_EXTENSION;
        }

        self.filename = Some(filename);

        return 0;
    }

    pub fn _to_metadata_json(&self) -> Result<String, serde_json::Error> {
        return serde_json::to_string_pretty(&self);
    }
}
*/

#[derive(Debug, Deserialize, Serialize)]
pub struct CoverArt {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub path: Option<String>,
    pub data: Option<Vec<u8>>,
}

impl Default for CoverArt {
    fn default() -> Self {
        CoverArt {
            id: None,
            title: None,
            path: None,
            data: None,
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
