use std::collections::HashMap;
use std::default::Default;
use std::fs::read_dir;
use std::io::{Result, Write};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

use crate::managers;
// use crate::models::song::Album;
use crate::models::{self};
use crate::parsers;
use crate::syncers;
use crate::utilities;

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

enum _RetrieveTypes {
    Songs,
}

#[derive(Clone, Debug)]
enum En {
    ImageFile,
    SongFile,
    MetadataFile,
    Other,
}

pub fn retrieve_song(
    album: &icarus_models::album::collection::Album,
    track: i32,
    disc: i32,
    directory: &String,
    filename: &String,
) -> Result<icarus_models::song::Song> {
    let mut found = false;
    let mut song = icarus_models::song::Song::default();

    for song_i in &album.tracks {
        if song_i.track == track && song_i.disc == disc {
            let track = song_i.clone();
            song.album = album.title.clone();
            song.album_artist = album.artist.clone();
            song.artist = track.artist.clone();
            song.audio_type = String::from(icarus_models::constants::DEFAULTMUSICEXTENSION);
            song.disc = track.disc.clone();
            song.disc_count = album.disc_count.clone();
            song.duration = track.duration as i32;
            song.genre = album.genre.clone();
            song.title = track.title.clone();
            song.year = album.year.clone();
            song.track = track.track.clone();
            song.track_count = album.track_count.clone();
            song.directory = directory.clone();
            song.filename = filename.clone();

            found = true;
            break;
        }
    }

    if found {
        return Ok(song);
    }

    return Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Song not found",
    ));
}

/*
impl Album {
    pub fn _print_info(&self) {
        println!("Album: {}", self.title);
        println!("Album Artist: {}", self.album_artist);
        println!("Genre: {}", self.genre);
        println!("Year: {}", self.year);
        println!("Track Count: {}", self.track_count);
        println!("Disc Count: {}\n", self.disc_count);
    }

    pub fn retrieve_song(&self, track: i32, disc: i32) -> Result<icarus_models::song::Song> {
        let mut found = false;
        let mut song = icarus_models::song::Song::default();

        for song_i in &self.songs {
            if song_i.track == track && song_i.disc == disc {
                song = song_i.clone();
                found = true;
            }
        }

        if found {
            return Ok(song);
        }

        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Song not found",
        ));
    }
}
    */

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

    fn delete_song(&self) {
        let mut prsr = parsers::api_parser::APIParser {
            ica_act: self.ica_action.clone(),
            api: models::api::API::default(),
        };
        prsr.parse_api();
        let api = prsr.retrieve_api();

        let token = self.parse_token(&api);

        println!("Deleting song");

        let mut song = icarus_models::song::Song::default();

        for arg in &self.ica_action.flags {
            let flag = &arg.flag;
            let value = &arg.value;

            if flag == "-D" {
                song.id = value.parse::<i32>().unwrap();
            }
        }

        let mut del = syncers::delete::Delete { api: api.clone() };

        println!("Deleting song..");

        let res_fut = del.delete_song(&token, &song);
        let result = Runtime::new().unwrap().block_on(res_fut);
        match result {
            Ok(o) => {
                println!("Song (Id {:?}) has been successfully deleted", o.id);
            }
            Err(er) => {
                println!("Error {:?}", er);
            }
        }
    }

    fn download_song(&self) {
        println!("Deleting song");
        let dwn = self.ica_action.retrieve_flag_value(&String::from("-b"));
        let id: i32 = dwn.parse::<i32>().unwrap();

        let mut prsr = parsers::api_parser::APIParser {
            api: models::api::API::default(),
            ica_act: self.ica_action.clone(),
        };
        prsr.parse_api();

        let api = prsr.retrieve_api();
        let token = self.parse_token(&api);
        println!("Message: {}", token.message);

        let mut dwn_loader = syncers::download::Download { api: api.clone() };
        let mut song = icarus_models::song::Song::default();
        song.id = id;
        let result_fut = dwn_loader.download_song(&token, &song);
        match Runtime::new().unwrap().block_on(result_fut) {
            Ok(o) => {
                println!("Success");
                let filename =
                    String::from("audio") + icarus_models::constants::DEFAULTMUSICEXTENSION;
                let data = o.as_bytes();
                let mut file = std::fs::File::create(filename).expect("Failed to save");
                file.write_all(&data)
                    .expect("Failed to save downloaded song");
            }
            Err(er) => {
                println!("Error {:?}", er);
                match er {
                    syncers::download::MyError::Request(error) => {
                        println!("Error: {:?}", error);
                    }
                    syncers::download::MyError::Other(ss) => {
                        println!("Error: {:?}", ss);
                    }
                }
            }
        }
    }

    fn retrieve_object(&self) {
        println!("Deleting song");
        let rt = self.ica_action.retrieve_flag_value(&String::from("-rt"));

        if rt != "songs" {
            panic!("Unsupported -rt: {}", rt);
        }

        let mut prsr = parsers::api_parser::APIParser {
            api: models::api::API::default(),
            ica_act: self.ica_action.clone(),
        };
        prsr.parse_api();

        let api = prsr.retrieve_api();
        let token = self.parse_token(&api);
        let mut repo = syncers::retrieve_records::RetrieveRecords { api: api.clone() };
        let result_fut = repo.get_all_songs(&token);

        let result = Runtime::new().unwrap().block_on(result_fut);
        match result {
            Ok(o) => {
                for _son in o {
                    // son.print_info();
                }
            }
            Err(er) => {
                println!("Error: {:?}", er);
            }
        }
    }

    fn upload_song(&self) {
        println!("Deleting song");
        panic!("Not supported");
    }

    fn parse_token(&self, api: &models::api::API) -> icarus_models::token::AccessToken {
        println!("Fetching token");

        let mut usr_mgr: managers::user_manager::UserManager =
            managers::user_manager::UserManager {
                user: icarus_models::user::User::default(),
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
            panic!("What??");
        }

        if single_target {
            println!("Song path: {}", songpath);
            println!("Track ID: {}", track_id);
            println!("metadata path: {}", metadata_path);
            println!("cover art path: {}", coverpath);

            let _ = self.sing_target_upload(&songpath, &track_id, &metadata_path, &coverpath);
        } else if multitarget {
            let _ = self.multi_target_upload(&uni);
        } else {
            println!("Single or Multi target has not been chosen");
        }
    }

    fn sing_target_upload(
        &mut self,
        songpath: &String,
        track_id: &String,
        meta_path: &String,
        cover_path: &String,
    ) -> Result<()> {
        let mut prsr = parsers::api_parser::APIParser {
            api: models::api::API::default(),
            ica_act: self.ica_action.clone(),
        };
        prsr.parse_api();

        let api = prsr.retrieve_api();
        let token = self.parse_token(&api);

        let song_file = std::path::Path::new(&songpath);

        if !song_file.exists() {
            println!("Song file does not exist");
            panic!("Error");
        }

        let mut cover_art = icarus_models::coverart::CoverArt {
            id: 0,
            title: String::new(),
            path: String::new(),
            data: Vec::new(),
        };
        let mut song = icarus_models::song::Song::default();
        let mut filenames = Vec::new();
        let mut fp = String::new();
        let mut dir = String::new();

        let entry = &song_file;

        let file_name = std::ffi::OsString::from(entry.file_name().unwrap());

        println!("file name: {:?}", file_name);

        match self.find_file_extension(&file_name) {
            En::ImageFile => {}
            En::MetadataFile => {}
            En::SongFile => {
                let fname = self.o_to_string(&file_name);

                match fname {
                    Ok(s) => {
                        filenames.push(s.clone());
                        fp = s.clone();
                        dir = song_file.parent().unwrap().display().to_string();
                        song.filename = s.clone();
                        song.directory = dir.clone();
                        self.initialize_disc_and_track(&mut song);
                    }
                    Err(er) => println!("Error: {:?}", er),
                }
            }
            _ => {}
        }

        cover_art.path = cover_path.clone();

        let album = self.retrieve_metadata(&meta_path);
        let trck = i32::from_str(track_id).unwrap();
        let mut s = retrieve_song(&album, trck, 1, &fp, &dir).unwrap();
        // s.filename = fp;
        // s.directory = dir;
        // s.genre = album.genre.clone();
        // s.year = album.year.clone();
        // s.album = album.title.clone();
        s.data = s.to_data().unwrap();

        cover_art.data = cover_art.to_data().unwrap();

        let mut up = syncers::upload::Upload::default();
        let host = self.ica_action.retrieve_flag_value(&String::from("-h"));
        up.set_api(&host);

        let res = up.upload_song_with_metadata(&token, &s, &cover_art, &album);
        let tken = Runtime::new().unwrap().block_on(res);

        match &tken {
            Ok(o) => {
                println!("Successfully sent {:?}", o);
            }
            Err(er) => {
                println!("Some error {:?}", er);
            }
        }

        Ok(())
    }

    fn get_songs(
        &self,
        metadata_path: &String,
        source_directory: &String,
    ) -> Result<Vec<icarus_models::song::Song>> {
        match icarus_models::album::collection::parse_album(metadata_path) {
            Ok(albums) => {
                for entry in read_dir(source_directory)? {}

                Ok(Vec::new())
            }
            Err(_) => {
                // Err()
                Ok(Vec::new())
            }
        }
    }

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
            panic!("Directory does not exist");
        }

        let mut cover_art = icarus_models::coverart::CoverArt {
            id: 0,
            title: String::new(),
            path: String::new(),
            data: Vec::new(),
        };
        let mut songs: Vec<icarus_models::song::Song> = Vec::new();
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
                    cover_art.path = fullpath;
                }
                En::MetadataFile => {
                    let directory_part = sourcepath.clone();
                    let fname = self.o_to_string(&file_name);
                    metadatapath = directory_part + "/" + &fname.unwrap();
                }
                En::SongFile => {
                    let mut song = icarus_models::song::Song::default();
                    let fname = self.o_to_string(&file_name);

                    match fname {
                        Ok(s) => {
                            filenames.push(s.clone());
                            song.filename = s.clone();
                            song.directory = sourcepath.clone();
                            song.data = song.to_data().unwrap();
                            self.initialize_disc_and_track(&mut song);

                            songs.push(song);
                        }
                        Err(er) => println!("Error: {:?}", er),
                    }
                }
                _ => {}
            }
        }

        filenames.sort();
        let songs = self.get_songs(&metadatapath, &sourcepath);

        let album = self.retrieve_metadata(&metadatapath);

        // self.song_parsing(&mut album, &sourcepath, &filenames);

        let mut up = syncers::upload::Upload::default();
        let host = self.ica_action.retrieve_flag_value(&String::from("-h"));
        up.set_api(&host);

        cover_art.data = cover_art.to_data().unwrap();

        println!("");

        match songs {
            Ok(sngs) => {
                for song in sngs {
                    let res = up.upload_song_with_metadata(&token, &song, &cover_art, &album);
                    // let tken =
                    match Runtime::new().unwrap().block_on(res) {
                        Ok(o) => {
                            println!("Response: {:?}", o);
                        }
                        Err(err) => {
                            println!("Error: {:?}", err);
                        }
                    };
                }
            }
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }

        /*
        for track in &mut album.tracks {
            match sng.to_data() {
                Ok(data) => {
                    sng.data = data;
                }
                Err(err) => {
                    return Err(err);
                }
            };
        }

        for track in &album.tracks {
            // Upload each song
            println!("Sending song...");
            // let song =
            match retrieve_song(&album, track.track, 1, &sourcepath, filename) {
                Ok(song) => {
                    let res = up.upload_song_with_metadata(&token, &song, &cover_art, &album);
                    let tken = Runtime::new().unwrap().block_on(res);

                    match &tken {
                        Ok(o) => {
                            println!("Successfully sent {:?}", o);
                        }
                        Err(er) => {
                            println!("Some error {:?}", er);
                        }
                    }

                    println!("");
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                }
            }
        }
        */

        Ok(())
    }

    // Makes sure the elements in album.songs is populated
    /*
    fn song_parsing(
        &self,
        album: &mut icarus_models::album::collection::Album,
        directory: &String,
        filenames: &Vec<String>,
    ) {
        // Apply directory
        for song in &mut album.tracks {
            // let dir = &song.directory;
            song.directory = directory.clone();
        }

        // Apply filename
        let mut index = 0;
        for song in &mut album.tracks {
            let filename = filenames[index].clone();
            song.filename = filename.clone();
            index += 1;
        }

        for song in &mut album.tracks {
            song.album = album.title.clone();
            song.genre = album.genre.clone();
            song.year = album.year.clone();
        }
    }
    */

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
    fn initialize_disc_and_track(&mut self, song: &mut icarus_models::song::Song) {
        let mut disc = 1;
        let mut track = 1;
        let mut mode = 0;
        let filename = &song.filename;

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

        song.disc = disc;
        song.track = track;
    }

    fn _parse_disc_and_track(&self, song: &mut icarus_models::song::Song, track_id: &String) {
        let sep = |_a: &char, _b: &char| -> bool {
            return false;
        };

        let index = utilities::checks::Checks::index_of_item_in_container(track_id, &':', sep);

        if index == -1 {
            let mut d_str: String = String::new();
            let t_str = String::new();

            for c in track_id.chars().skip(0).take(index as usize) {
                d_str.push(c);
            }

            let start = index + 1;
            let end = track_id.len() - 1;

            for c in track_id.chars().skip(start as usize).take(end as usize) {
                d_str.push(c);
            }

            song.disc = d_str.parse::<i32>().unwrap();
            song.track = t_str.parse::<i32>().unwrap();
        } else {
            if utilities::checks::Checks::is_numeric(track_id) {
                song.track = track_id.parse::<i32>().unwrap();
            }
        }
    }

    fn _check_for_no_confirm(&self) -> bool {
        for flag in self.ica_action.flags.iter() {
            if flag.flag == "-nc" {
                return true;
            }
        }
        return false;
    }

    fn retrieve_metadata(&self, path: &String) -> icarus_models::album::collection::Album {
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
