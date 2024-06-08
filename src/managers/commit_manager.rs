use std::collections::HashMap;
use std::default::Default;
use std::fmt::Display;
use std::fs::{read_dir, DirEntry};
use std::io::{Error, Read, Result};
use std::path::Path;

use futures::{FutureExt, TryFutureExt};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

use crate::models;
use crate::models::song::Album;
use crate::parsers;
use crate::syncers;
use crate::utilities;
use crate::{exit_program, managers};

#[derive(Debug, Deserialize, Serialize)]
pub struct CommitManager {
    pub ica_action: models::icarus_action::IcarusAction,
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

#[derive(Clone, Debug)]
enum En {
    ImageFile,
    SongFile,
    MetadataFile,
    Other,
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
    pub fn commit_action(&mut self) {
        let action = &self.ica_action.action;
        println!("Committing {} action", action);

        let mapped_actions = &self.map_actions();
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
            }
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

        let del = syncers::delete::Delete { api: api };

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
        let mut tok_mgr = managers::token_manager::TokenManager {
            user: usr,
            api: api.clone(),
        };
        tok_mgr.init();

        let token = Runtime::new().unwrap().block_on(tok_mgr.request_token());

        return token.unwrap();
    }

    // TODO: Implement
    fn upload_song_with_metadata(&mut self) {
        println!("Uplodaring song with metadara");

        let songpath = self.ica_action.retrieve_flag_value(&String::from("-s"));
        let metadata_path = self.ica_action.retrieve_flag_value(&String::from("-m"));
        let coverpath = self.ica_action.retrieve_flag_value(&String::from("-ca"));
        let track_id = self.ica_action.retrieve_flag_value(&String::from("-t"));

        let single_target = songpath.len() > 0
            && metadata_path.len() > 0
            && coverpath.len() > 0
            && track_id.len() > 0;

        let uni = self.ica_action.retrieve_flag_value(&String::from("-smca"));
        let multitarget = uni.len() > 0;

        if single_target && multitarget {
            println!("Cannot upload from source and directory");
        }

        if single_target {
            println!("Song path: {}", songpath);
            println!("Track ID: {}", track_id);
            println!("metadata path: {}", coverpath);
            println!("cover art path: {}", track_id);

            self.sing_target_upload(&songpath, &track_id, &metadata_path, &coverpath);
        } else if multitarget {
            self.multi_target_upload(&uni);
        } else {
            println!("Single or Multi target has not been chosen");
        }
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
    fn multi_target_upload(&mut self, sourcepath: &String) -> std::io::Result<()> {
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

        let mut cover_art = models::song::CoverArt::default();
        let mut songs: Vec<models::song::Song> = Vec::new();
        let mut filenames: Vec<String> = Vec::new();
        let mut metadatapath: String = String::new();

        // iterate files in metadatapath
        let path = std::path::Path::new(directory_path);

        for entry in read_dir(path)? {
            let entry = entry?;

            let file_type = entry.file_type();
            let file_name = entry.file_name();

            println!("file type: {:?}", file_type);
            println!("file name: {:?}", file_name);


            match self.find_file_extension(&file_name) {
                En::ImageFile => {
                    let directory_part = sourcepath.clone();
                    let fname = self.o_to_string(&file_name);
                    let fullpath = directory_part + "/" + &fname.unwrap();
                    cover_art.path = Some(fullpath);
                }
                En::MetadataFile => {
                    let directory_part = sourcepath.clone();
                    let fname = self.o_to_string(&file_name);
                    metadatapath = directory_part + "/" + &fname.unwrap();
                }
                En::SongFile => {
                    let mut song = models::song::Song::default();
                    let fname = self.o_to_string(&file_name);

                    match fname {
                        Ok(s) => {
                            filenames.push(s.clone());
                            song.filepath = Some(s.clone());
                            song.directory = Some(sourcepath.clone());
                            self.initialize_disc_and_track(&mut song);
                        }
                        Err(er) => println!("Error: {:?}", er),
                    }

                    songs.push(song)
                }
                _ => {}
            }
        }

        filenames.sort();

        let mut album = self.retrieve_metadata(&metadatapath);

        self.song_parsing(&mut album, &sourcepath, &filenames);


        let mut up = syncers::upload::Upload::default();
        let host = self.ica_action.retrieve_flag_value(&String::from("-h"));
        up.set_api(&host);

        println!("");

        for song in &album.songs {
            // Upload each song
            println!("Sending song...");
            let res = up.upload_song_with_metadata(&token, &song, &cover_art, &album);
            let tken = Runtime::new().unwrap().block_on(res);

            match &tken {
                Ok(o) => {
                    println!("Successfully sent {:?}", o);
                },
                Err(er) => {
                    println!("Some error {:?}", er);
                }
            }
            
            println!("");
        }

        Ok(())
    }

    // Makes sure the elements in album.songs is populated
    fn song_parsing(
        &self,
        album: &mut models::song::Album,
        directory: &String,
        filenames: &Vec<String>,
    ) {
        // Apply directory
        for song in &mut album.songs {
            let dir = &song.directory;
            match dir {
                Some(s) => println!("{}", s),
                None => {
                    song.directory = Some(directory.clone());
                }
            }
        }

        // Apply filename
        let mut index = 0;
        for song in &mut album.songs {
            let filename = filenames[index].clone();
            song.filepath = Some(filename);
            index += 1;
        }

        for song in &mut album.songs {
            match &mut song.album {
                Some(_) => {}
                None => {
                    song.album = Some(album.title.clone());
                }
            }

            match &mut song.genre {
                Some(_) => {}
                None => {
                    song.genre = Some(album.genre.clone());
                }
            }

            match &mut song.year {
                Some(_) => {}
                None => {
                    song.year = Some(album.year.clone());
                }
            }
        }
    }

    fn find_file_extension(&self, file_name: &std::ffi::OsString) -> En {
        let file_name_str = Some(file_name.clone().into_string());

        match file_name_str {
            Some(string) => {
                let a = string.unwrap();
                let split = a.split(".");
                let mut last_index = 0;

                for _ in split.clone() {
                    last_index += 1;
                }

                let mut extension = String::new();
                let mut index = 1;

                for word in split {
                    if index == last_index {
                        extension = word.to_string();
                        break;
                    }

                    index += 1;
                }

                if extension == "wav" || extension == "flac" {
                    return En::SongFile;
                } else if extension == "json" {
                    return En::MetadataFile;
                } else if extension == "jpg" || extension == "jpeg" || extension == "png" {
                    return En::ImageFile;
                }
            }
            _ => {
                return En::Other;
            }
        }

        return En::Other;
    }

    fn o_to_string(&self, val: &std::ffi::OsString) -> Result<std::string::String> {
        let res = val.clone().into_string();
        return match res {
            Ok(sss) => Ok(sss),
            Err(_) => Ok(String::from("Error")),
        };
    }

    // Standards
    // * track01.cdda.wav - Disc 1, Track 1
    // * track02d02.cdda.wav - Disc 2, Track 2
    fn initialize_disc_and_track(&mut self, song: &mut models::song::Song) {
        let mut disc = 1;
        let mut track = 1;
        let mut mode = 0;
        let filename =
            &<std::option::Option<std::string::String> as Clone>::clone(&song.filepath).unwrap();

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
                        if index >= st && index <= ed {
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
        let content = self.retrieve_file_content(&path);
        let val = content.unwrap();

        let converted = serde_json::from_str(&val);

        match &converted {
            Ok(_) => println!("Good!"),
            Err(er) => println!("Error {:?}", er),
        }
        return converted.unwrap();
    }

    fn retrieve_file_content(&self, path: &String) -> Result<String> {
        return std::fs::read_to_string(path);
    }
}
