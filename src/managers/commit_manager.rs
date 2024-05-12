use std::collections::HashMap;
use std::default::Default;
use std::fs::File;
use std::io::{Read, Error};
use tokio::runtime::Runtime;

use futures::{FutureExt, TryFutureExt};
use serde::{Deserialize, Serialize};

use crate::{exit_program, managers};
use crate::models;
use crate::parsers;
use crate::syncers;
use crate::utilities;

#[derive(Debug, Deserialize, Serialize)]
pub struct CommitManager {
    pub ica_action: models::icarus_action::IcarusAction,
}

#[derive(Debug, Deserialize, Serialize)]
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
        println!("Disc Count: {}\n", self.disc_count);
    }
}

impl CommitManager {
    pub fn commit_action(&self) {
        let action = &self.ica_action.action;
        println!("Committing {} action", action);

        let mapped_actions = self.map_actions();
        let mapped_action = self.find_mapped_action(&mapped_actions, action);

        println!("{:?}", mapped_action);

        match mapped_action {
            ActionValues::DeleteAct => self.delete_song(),
            ActionValues::DownloadAct => self.download_song(),
            ActionValues::RetrieveAct => self.retrieve_object(),
            ActionValues::UploadAct => self.upload_song(),
            ActionValues::UploadSongWithMetadata => self.upload_song_with_metadata(),
            _ => {
                println!("Nothing good here");
            },
        }
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
        actions.insert("upload".to_string(), ActionValues::UploadAct);
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
        prsr.parse_api();
        let api = prsr.retrieve_api();

        let token = self.parse_token(&api);

        println!("Deleting song");

        let mut song = models::song::Song::default();

        for arg in &self.ica_action.flags {
            let flag = &arg.flag;
            let value = &arg.value;

            if flag == "-D" {
                song.id = Some(value.parse::<i32>().unwrap());
            }
        }

        let del = syncers::delete::Delete {
            api: api,
        };

        println!("Deleting song..");

        del.delete_song(&token, &song);
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
        usr_mgr.parse_user_from_actions();

        let usr = usr_mgr.retrieve_user();
        let tok_mgr = managers::token_manager::TokenManager {
            user: usr,
            api: api.clone(),
        };

        // let token = tok_mgr.request_token();
        let token = Runtime::new().unwrap().block_on(tok_mgr.request_token());

        return token.unwrap();
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
    fn multi_target_upload(&self, sourcepath: &String) {
        let mut prsr = parsers::api_parser::APIParser {
            api: models::api::API::default(),
            ica_act: self.ica_action.clone(),
        };
        prsr.parse_api();
        let api = prsr.retrieve_api();
        let token = self.parse_token(&api);

        let directory_path = std::path::Path::new(&sourcepath);

        if !directory_path.exists() {
            println!("Directory does not exist");
            std::process::exit(-1);
        }

        // let mut cover_art = models::
    }

    // Standards
    // * track01.cdda.wav - Disc 1, Track 1
    // * track02d02.cdda.wav - Disc 2, Track 2
    fn initialize_disc_and_track(&mut self, song: &mut models::song::Song) {
        let mut disc = 1;
        let mut track = 1;
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

                    if utilities::checks::Checks::is_numeric(&t) {
                        track = t.parse::<i32>().unwrap();
                    }
                    disc = 1
                }
            }
            2 => {
                if k != end && dot != end && d != end {
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

                    if utilities::checks::Checks::is_numeric(&t) {
                        track = t.parse::<i32>().unwrap();
                    }

                    let sst = d + 1;
                    let eed = dot;
                    let mut d_s = String::new();
                    index = 0;
                    for a in filename.chars() {
                        if index <= eed {
                            d_s.push(a);
                        } else if index >= sst {
                            d_s.push(a);
                        }

                        index += 1;
                    }

                    if utilities::checks::Checks::is_numeric(&d_s) {
                        track = d_s.parse::<i32>().unwrap();
                    }
                }
            }
            _ => println!(""),
        }

        song.disc = Some(disc);
        song.track = Some(track);
    }

    fn parse_disc_and_track(&self, song: &mut models::song::Song, track_id: &String) {
        let sep = |a: &char, b: &char| -> bool {
            return false;
        };

        let index = utilities::checks::Checks::index_of_item_in_container(track_id, &':', sep);

        if index == -1 {
            let mut d_str: String = String::new();
            let mut t_str = String::new();

            for c in track_id.chars().skip(0).take(index as usize) {
                d_str.push(c);
            }

            let start = index + 1;
            let end = track_id.len() - 1;

            for c in track_id.chars().skip(start as usize).take(end as usize) {
                d_str.push(c);
            }

            song.disc = Some(d_str.parse::<i32>().unwrap());
            song.track = Some(t_str.parse::<i32>().unwrap());
        } else {
            if utilities::checks::Checks::is_numeric(track_id) {
                song.track = Some(track_id.parse::<i32>().unwrap());
            }
        }
    }

    fn check_for_no_confirm(&self) -> bool {
        for flag in self.ica_action.flags.iter() {
            if flag.flag == "-nc" {
                return true;
            }
        }
        return false;
    }

    fn retrieve_metadata(&self, path: &String) -> Album {
        /*
        let mut alb = Album {
            title: String::from(""),
            album_artist: String::from(""),
            genre: String::from(""),
            year: 0,
            track_count: 0,
            disc_count: 0,
            songs: Vec::new(),
        };
        */

        let content = self.retrieve_file_content(&path);
        // let alb = serde_json::from_str(&content.unwrap());

            /*
            let mut file: std::fs::File = std::fs::File::open(filepath).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    return serde_json::from_str(&data).unwrap();
    */

        // return alb.unwrap();
        return serde_json::from_str(&content.unwrap()).unwrap();
    }

    fn retrieve_file_content(&self, path: &String) -> Result<String, Error> {
        /*
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        */

        return std::fs::read_to_string(path);
    }
}
