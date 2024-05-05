use std::collections::HashMap;
use std::default::Default;

use serde::{Deserialize, Serialize};

use crate::managers;
use crate::models;
use crate::parsers;
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

#[derive(Clone, Debug)]
enum ActionValues {
    DeleteAct,
    DownloadAct,
    RetrieveAct,
    UploadAct,
    UploadSongWithMetadata,
    None,
}

enum RetrieveTypes {
    Songs,
}

impl Album {
    pub fn print_info(&self) {
        println!("Album: {}", self.title);
        println!("Album Artist: {}", self.album_artist);
        println!("Genre: {}", self.genre);
        println!("Year: {}", self.year);
        println!("Track Count: {}", self.track_count);
        println!("Disc Count: {}", self.disc_count);
    }
}

impl CommitManager {
    pub fn commit_action(&self) {
        let action = &self.ica_action.action;
        println!("Committing {} action", action);

        let mapped_actions = self.map_actions();
        let mapped_action = self.find_mapped_action(&mapped_actions, action);

        println!("{:?}", mapped_action);
    }

    fn find_mapped_action(
        &self,
        actions: &HashMap<String, ActionValues>,
        action: &String,
    ) -> ActionValues {
        for (key, act) in actions {
            if key == action {
                return act.clone();
            }
        }

        return ActionValues::None;
    }

    fn map_actions(&self) -> HashMap<String, ActionValues> {
        let mut actions: HashMap<String, ActionValues> = HashMap::new();
        actions.insert("download".to_string(), ActionValues::DownloadAct);
        // actions.insert("upload".to_string(), ActionValues::UploadAct);
        actions.insert(
            "upload-meta".to_string(),
            ActionValues::UploadSongWithMetadata,
        );
        actions.insert("retrieve".to_string(), ActionValues::RetrieveAct);
        actions.insert("delete".to_string(), ActionValues::DeleteAct);

        return actions;
    }

    // TODO: Implement
    fn delete_song(&self) {
        let mut prsr = parsers::api_parser::APIParser {
            ica_act: self.ica_action.clone(),
            api: models::api::API::default(),
        };

        println!("Deleting song");

        let api = prsr.retrieve_api();

        let song = models::song::Song::default();

        for arg in &self.ica_action.flags {}
    }

    // TODO: Implement
    fn download_song(&self) {
        println!("Deleting song");
    }

    // TODO: Implement
    fn retrieve_object(&self) {
        println!("Deleting song");
    }

    // TODO: Implement
    // NOTE: Might not need to implement. I will see how this goes
    fn upload_song(&self) {
        println!("Deleting song");
    }

    fn parse_token(&self, api: &models::api::API) -> models::token::Token {
        println!("Fetching token");

        let mut usr_mgr: managers::user_manager::UserManager =
            managers::user_manager::UserManager {
                user: models::user::User {
                    username: String::new(),
                    password: String::new(),
                },
                ica_action: self.ica_action.clone(),
            };

        let usr = usr_mgr.retrieve_user();
        let tok_mgr = managers::token_manager::TokenManager {
            user: usr,
            api: api.clone(),
        };

        return tok_mgr.request_token();
    }
    // TODO: Implement
    fn upload_song_with_metadata(&self) {
        println!("Deleting song");
    }
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
    // Standards
    // * track01.cdda.wav - Disc 1, Track 1
    // * track02d02.cdda.wav - Disc 2, Track 2
    fn initialize_disc_and_track(&mut self, song: &mut models::song::Song) {
        let disc = 1;
        let track = 1;
        let mut mode = 0;
        let songpath = song.song_path();
        let filename =
            &<std::option::Option<std::string::String> as Clone>::clone(&song.filepath).unwrap();

        // let directory = &<std::option::Option<std::string::String> as Clone>::clone(&self.directory).unwrap();
        let trd = filename.contains("trackd");
        let tr = filename.contains("track");

        if tr {
            mode = 1;
        }

        if trd {
            mode = 2;
        }

        let dl = |a: &char, b: &char| -> bool {
            return a == b;
        };
        let d = utilities::checks::Checks::index_of_item_in_container(&filename, &'d', dl);
        let k = utilities::checks::Checks::index_of_item_in_container(&filename, &'k', dl);
        let dot = utilities::checks::Checks::index_of_item_in_container(&filename, &'.', dl);
        let end = filename.len() as i32;

        match mode {
            1 => {
                if k != end && dot != end {
                    let st = k + 1;
                    let ed = dot - 1;
                    let mut t: String = String::new();
                    let mut index = 0;
                    for a in filename.chars() {
                        if index <= ed {
                            t.push(a);
                        } else if index >= st {
                            t.push(a);
                        }

                        index += 1;
                    }
                    // let t = &filename[1..5];
                }
            }
            2 => {}
            _ => println!(""),
        }

        song.disc = Some(disc);
        song.track = Some(track);
    }
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
