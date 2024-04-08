use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Song {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub genre: String,
    pub year: int,
    pub duration: f64,
    pub track: i32,
    pub disc: i32,
    pub data: String,
    // use filepath instead
    // pub song_path: String,
    pub filepath: String,
    pub directory: String,
}

impl Song {
    // TODO: Implement
    pub fn print_info(&self) {
    }

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
struct CoverArt {
    pub id: i32,
    pub title: String,
    pub path: String,
}