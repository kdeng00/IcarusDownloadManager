use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models;
use crate::utilities;

#[derive(Debug, Deserialize, Serialize)]
pub struct CommitManager {
    // pub action: String,
    // pub flags: Vec<models::flags::Flags>,
    // pub params: Vec<String>,
    // pub param_count: i32,
    pub ica_action: models::icarus_action::IcarusAction,
}

pub struct Album {
    pub title: String,
    pub album_artist: String,
    pub genre: String,
    pub year: i32,
    pub track_count: i32,
    pub disc_count: i32,
    pub songs: Vec<models::song::Song>,
}

enum ActionValues {
    DeleteAct,
    DownloadAct,
    RetrieveAct,
    UploadAct,
    UploadSongWithMetadata,
}

enum RetrieveTypes {
    Songs,
}

impl Album {
    // TODO: Implement
    pub fn print_info(&self) {}
}

impl CommitManager {
    // TODO: Implement
    pub fn commit_action(&self) {}

    // TODO: Implement
    fn map_actions(&self) -> HashMap<String, ActionValues> {
        return HashMap::new();
    }

    // TODO: Implement
    fn parse_token(&self, api: &models::api::API) {}
    // TODO: Implement
    fn upload_song_with_metadata(&self) {}
    // TODO: Implement
    fn sing_target_upload(
        &self,
        songpath: &String,
        track_id: &String,
        meta_path: &String,
        cover_path: &String,
    ) {
    }
    // TODO: Implement
    fn multi_target_upload(&self, sourcepath: &String) {}
    // TODO: Implement
    fn initialize_disc_and_track(&self, song: &models::song::Song) {}
    // TODO: Implement
    fn parse_disc_and_track(&self, song: &models::song::Song, track_id: &String) {}
    // TODO: Implement
    fn check_for_no_confirm(&self) -> bool {
        return false;
    }
    // TODO: Implement
    fn retrieve_metadata(&self, path: &String) -> Album {
        return Album {
            title: String::from(""),
            album_artist: String::from(""),
            genre: String::from(""),
            year: 0,
            track_count: 0,
            disc_count: 0,
            songs: Vec::new(),
        };
    }
    // TODO: Implement
    fn retrieve_file_content(&self, path: &String) -> String {
        return String::from("");
    }
}
