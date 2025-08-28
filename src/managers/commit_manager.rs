use std::collections::HashMap;
use std::default::Default;
use std::fs::read_dir;
use std::io::{Result, Write};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

use crate::managers;
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

#[derive(Clone, Debug)]
enum En {
    ImageFile,
    SongFile,
    MetadataFile,
    Other,
}

#[derive(Debug)]
struct UploadSongMembers {
    pub song: icarus_models::song::Song,
    pub coverart: icarus_models::coverart::CoverArt,
    pub token: icarus_models::token::AccessToken,
    pub album: icarus_models::album::collection::Album,
}

pub fn retrieve_song(
    album: &icarus_models::album::collection::Album,
    track: i32,
    disc: i32,
    directory: &str,
    filename: &str,
) -> Result<icarus_models::song::Song> {
    let mut found = false;
    let mut song = icarus_models::song::Song::default();

    for song_i in &album.tracks {
        if song_i.track == track && song_i.disc == disc {
            let track = song_i.clone();
            song.album = album.title.clone();
            song.album_artist = album.artist.clone();
            song.artist = track.artist.clone();
            song.audio_type = String::from(
                icarus_models::constants::file_extensions::audio::DEFAULTMUSICEXTENSION,
            );
            song.disc = track.disc;
            song.disc_count = album.disc_count;
            song.duration = track.duration as i32;
            song.genre = album.genre.clone();
            song.title = track.title.clone();
            song.year = album.year;
            song.track = track.track;
            song.track_count = album.track_count;
            song.directory = directory.to_owned();
            song.filename = filename.to_owned();

            found = true;
            break;
        }
    }

    if found {
        Ok(song)
    } else {
        Err(std::io::Error::other("Song not found"))
    }
}

impl CommitManager {
    pub async fn commit_action(&mut self) {
        let action = &self.ica_action.action;
        println!("Committing {action} action");

        let mapped_actions = &self.map_actions();
        let mapped_action = self.find_mapped_action(mapped_actions, action);

        println!("{mapped_action:?}");

        // TODO: Move code to get token here and then pass it to the respective functions

        match mapped_action {
            ActionValues::DeleteAct => self.delete_song().await,
            ActionValues::DownloadAct => self.download_song().await,
            ActionValues::RetrieveAct => self.retrieve_object().await,
            ActionValues::UploadAct => self.upload_song(),
            ActionValues::UploadSongWithMetadata => self.upload_song_with_metadata().await,
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

        ActionValues::None
    }

    fn map_actions(&self) -> HashMap<String, ActionValues> {
        let actions: HashMap<String, ActionValues> = HashMap::from([
            ("download".to_string(), ActionValues::DownloadAct),
            ("download".to_string(), ActionValues::DownloadAct),
            ("upload".to_string(), ActionValues::UploadAct),
            (
                "upload-meta".to_string(),
                ActionValues::UploadSongWithMetadata,
            ),
            ("retrieve".to_string(), ActionValues::RetrieveAct),
            ("delete".to_string(), ActionValues::DeleteAct),
        ]);

        actions
    }

    async fn delete_song(&self) {
        let mut prsr = parsers::api_parser::APIParser {
            apis: vec![models::api::Api::default(), models::api::Api::default()],
            ica_act: self.ica_action.clone(),
        };
        prsr.parse_api(parsers::api_parser::APIType::Main);
        prsr.parse_api(parsers::api_parser::APIType::Auth);
        let auth_api = prsr.retrieve_api(parsers::api_parser::APIType::Auth);

        let token = self.parse_token(&auth_api).await;

        println!("Deleting song");

        let mut song = icarus_models::song::Song::default();

        for arg in &self.ica_action.flags {
            let flag = &arg.flag;
            let value = &arg.value;

            if flag == "-D" {
                song.id = uuid::Uuid::from_str(value.as_str()).unwrap();
            }
        }

        let api = prsr.retrieve_api(parsers::api_parser::APIType::Main);
        let mut del = syncers::delete::Delete { api: api.clone() };

        println!("Deleting song..");

        let res_fut = del.delete_song(&token, &song);
        let result = Runtime::new().unwrap().block_on(res_fut);
        match result {
            Ok(o) => {
                println!("Song (Id {:?}) has been successfully deleted", o.id);
            }
            Err(er) => {
                println!("Error {er:?}");
            }
        }
    }

    async fn download_song(&self) {
        println!("Deleting song");
        let dwn = self.ica_action.retrieve_flag_value(&String::from("-b"));
        let song_id = uuid::Uuid::from_str(dwn.as_str()).unwrap();

        let mut prsr = parsers::api_parser::APIParser {
            apis: vec![models::api::Api::default(), models::api::Api::default()],
            ica_act: self.ica_action.clone(),
        };
        prsr.parse_api(parsers::api_parser::APIType::Main);
        prsr.parse_api(parsers::api_parser::APIType::Auth);

        let auth_api = prsr.retrieve_api(parsers::api_parser::APIType::Auth);
        let token = self.parse_token(&auth_api).await;
        println!("Message: {}", token.message);
        let api = prsr.retrieve_api(parsers::api_parser::APIType::Main);

        let mut dwn_loader = syncers::download::Download { api: api.clone() };
        let song = icarus_models::song::Song {
            id: song_id,
            ..Default::default()
        };
        let result_fut = dwn_loader.download_song(&token, &song);
        match Runtime::new().unwrap().block_on(result_fut) {
            Ok(o) => {
                println!("Success");
                let filename = String::from("audio")
                    + icarus_models::constants::file_extensions::audio::DEFAULTMUSICEXTENSION;
                let data = o.as_bytes();
                let mut file = std::fs::File::create(filename).expect("Failed to save");
                file.write_all(data)
                    .expect("Failed to save downloaded song");
            }
            Err(er) => {
                println!("Error {er:?}");
                match er {
                    syncers::download::MyError::Request(error) => {
                        println!("Error: {error:?}");
                    }
                    syncers::download::MyError::Other(ss) => {
                        println!("Error: {ss:?}");
                    }
                }
            }
        }
    }

    async fn retrieve_object(&self) {
        println!("Retrieving song");
        let rt = self.ica_action.retrieve_flag_value(&String::from("-rt"));

        if rt != "songs" {
            panic!("Unsupported -rt: {rt}");
        }

        let mut prsr = parsers::api_parser::APIParser {
            apis: vec![models::api::Api::default(), models::api::Api::default()],
            ica_act: self.ica_action.clone(),
        };
        prsr.parse_api(parsers::api_parser::APIType::Main);
        prsr.parse_api(parsers::api_parser::APIType::Auth);

        let auth_api = prsr.retrieve_api(parsers::api_parser::APIType::Auth);
        let token = self.parse_token(&auth_api).await;
        println!("Token {token:?}");

        let api = prsr.retrieve_api(parsers::api_parser::APIType::Main);

        let mut repo = syncers::retrieve_records::RetrieveRecords { api: api.clone() };
        let result_fut = repo.get_all_songs(&token);

        match Runtime::new().unwrap().block_on(result_fut) {
            Ok(o) => {
                for song in o {
                    println!("Title: {:?}", song.title);
                    println!("Artist: {:?}", song.artist);
                    println!("Album: {:?}", song.album);
                    println!("Year: {:?}", song.year);
                }
            }
            Err(er) => {
                println!("Error: {er:?}");
            }
        }
    }

    // TODO: Remove at some point
    fn upload_song(&self) {
        println!("Deleting song");
        panic!("Not supported");
    }

    async fn parse_token(&self, api: &models::api::Api) -> icarus_models::token::AccessToken {
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

        let token = tok_mgr.request_token().await;

        token.unwrap()
    }

    async fn upload_song_with_metadata(&mut self) {
        println!("Uplodaring song with metadara");

        let songpath = self.ica_action.retrieve_flag_value(&String::from("-s"));
        let metadata_path = self.ica_action.retrieve_flag_value(&String::from("-m"));
        let coverpath = self.ica_action.retrieve_flag_value(&String::from("-ca"));
        let track_id = self.ica_action.retrieve_flag_value(&String::from("-t"));

        let single_target = !songpath.is_empty()
            && !metadata_path.is_empty()
            && !coverpath.is_empty()
            && !track_id.is_empty();

        let uni = self.ica_action.retrieve_flag_value(&String::from("-smca"));
        let multitarget = !uni.is_empty();

        if single_target && multitarget {
            println!("Cannot upload from source and directory");
            panic!("What??");
        }

        if single_target {
            println!("Song path: {songpath}");
            println!("Track ID: {track_id}");
            println!("metadata path: {metadata_path}");
            println!("cover art path: {coverpath}");

            let _ = self
                .sing_target_upload(&songpath, &track_id, &metadata_path, &coverpath)
                .await;
        } else if multitarget {
            let _ = self.multi_target_upload(&uni).await;
        } else {
            println!("Single or Multi target has not been chosen");
        }
    }

    async fn sing_target_upload(
        &mut self,
        songpath: &String,
        track_id: &str,
        meta_path: &String,
        cover_path: &str,
    ) -> Result<()> {
        let mut prsr = parsers::api_parser::APIParser {
            apis: vec![models::api::Api::default(), models::api::Api::default()],
            ica_act: self.ica_action.clone(),
        };
        prsr.parse_api(parsers::api_parser::APIType::Main);
        prsr.parse_api(parsers::api_parser::APIType::Auth);

        let auth_api = prsr.retrieve_api(parsers::api_parser::APIType::Auth);
        let token = self.parse_token(&auth_api).await;

        println!("Token: {:?}", token.token);

        let song_file = std::path::Path::new(&songpath);

        if !song_file.exists() {
            println!("Song file does not exist");
            panic!("Error");
        }

        let mut cover_art = icarus_models::coverart::CoverArt {
            id: uuid::Uuid::nil(),
            title: String::new(),
            path: cover_path.to_owned(),
            data: Vec::new(),
            ..Default::default()
        };
        let file_name = std::ffi::OsString::from(&song_file.file_name().unwrap());

        match self.find_file_extension(&file_name) {
            En::SongFile => match utilities::string::o_to_string(&file_name) {
                Ok(s) => {
                    println!("file name: {file_name:?}");

                    match icarus_models::album::collection::parse_album(meta_path) {
                        Ok(album) => {
                            let filename = s.clone();
                            let directory = song_file.parent().unwrap().display().to_string();
                            let trck = i32::from_str(track_id).unwrap();
                            let mut s =
                                retrieve_song(&album, trck, 1, &directory, &filename).unwrap();
                            println!("Directory: {:?}", s.directory);
                            println!("Filename: {:?}", s.filename);
                            println!("Path: {:?}", s.song_path());
                            s.data = s.to_data().unwrap();

                            cover_art.data = cover_art.to_data().unwrap();

                            let members = UploadSongMembers {
                                song: s,
                                coverart: cover_art,
                                token: token,
                                album: album,
                            };

                            match self.upload_song_process(&members).await {
                                Ok(_) => Ok(()),
                                Err(err) => Err(err),
                            }
                        }
                        Err(err) => {
                            println!("Error: {err:?}");
                            Err(std::io::Error::other(err.to_string()))
                        }
                    }
                }
                Err(er) => {
                    println!("Error: {er:?}");
                    Err(std::io::Error::other(er.to_string()))
                }
            },
            _ => Err(std::io::Error::other("No sutitable file found".to_owned())),
        }
    }

    async fn upload_song_process(&self, data: &UploadSongMembers) -> Result<()> {
        let mut up = syncers::upload::Upload::default();
        let host = self.ica_action.retrieve_flag_value(&String::from("-h"));
        up.set_api(&host);

        let token = &data.token;
        let song = &data.song;
        let album = &data.album;
        let coverart = &data.coverart;

        println!("Queueing song");

        let queued_song_id = match up.queue_song(token, song).await {
            Ok(id) => id,
            Err(err) => {
                return Err(std::io::Error::other(err.to_string()));
            }
        };

        println!("Queued song Id: {queued_song_id:?}");

        match up.link_user_to_queued_song(token, &queued_song_id).await {
            Ok(_) => {
                println!("Queued song linked to user");
            }
            Err(err) => {
                return Err(std::io::Error::other(err.to_string()));
            }
        }

        let queued_metadata_id = match up.queue_metadata(token, album, song, &queued_song_id).await
        {
            Ok(id) => id,
            Err(err) => {
                return Err(std::io::Error::other(err.to_string()));
            }
        };

        println!("Queued metadata Id: {queued_metadata_id:?}");

        match up.queue_coverart(token, coverart).await {
            Ok(id) => match up.link_queued_song_to_queued_coverart(token, &queued_song_id, &id).await {
                Ok(_) => {
                    println!("Queued coverart Id: {id:?}");
                    println!("Linked queued song to queued coverart");
                }
                Err(err) => {
                    return Err(std::io::Error::other(err.to_string()));
                }
            }
            Err(err) => {
                return Err(std::io::Error::other(err.to_string()));
            }
        }

        Ok(())
    }

    fn get_songs(
        &self,
        metadata_path: &String,
        source_directory: &str,
    ) -> Result<Vec<icarus_models::song::Song>> {
        match icarus_models::album::collection::parse_album(metadata_path) {
            Ok(albums) => {
                let mut songs: Vec<icarus_models::song::Song> = Vec::new();

                for track in &albums.tracks {
                    let song_filename = if track.track < 10 {
                        "track0".to_owned()
                            + &track.track.to_string()
                            + icarus_models::constants::file_extensions::audio::DEFAULTMUSICEXTENSION
                    } else {
                        "track".to_owned()
                            + &track.track.to_string()
                            + icarus_models::constants::file_extensions::audio::DEFAULTMUSICEXTENSION
                    };

                    songs.push(icarus_models::song::Song {
                        id: uuid::Uuid::nil(),
                        title: track.title.clone(),
                        artist: track.artist.clone(),
                        disc: track.disc,
                        track: track.track,
                        duration: track.duration as i32,
                        year: albums.year,
                        album_artist: albums.artist.clone(),
                        genre: albums.genre.clone(),
                        disc_count: albums.disc_count,
                        track_count: albums.track_count,
                        album: albums.title.clone(),
                        audio_type: String::from("FLAC"),
                        directory: source_directory.to_owned(),
                        filename: song_filename,
                        user_id: uuid::Uuid::nil(),
                        data: Vec::new(),
                        date_created: String::new(),
                    });
                }
                Ok(songs)
            }
            Err(_) => Ok(Vec::new()),
        }
    }

    async fn multi_target_upload(&mut self, sourcepath: &String) -> std::io::Result<()> {
        let mut prsr = parsers::api_parser::APIParser {
            apis: vec![models::api::Api::default(), models::api::Api::default()],
            ica_act: self.ica_action.clone(),
        };
        prsr.parse_api(parsers::api_parser::APIType::Main);
        prsr.parse_api(parsers::api_parser::APIType::Auth);
        let auth_api = prsr.retrieve_api(parsers::api_parser::APIType::Auth);
        let token = self.parse_token(&auth_api).await;

        let directory_path = std::path::Path::new(&sourcepath);

        if !directory_path.exists() {
            panic!("Directory does not exist");
        }

        let coverart_path = self.get_cover_art_path(sourcepath).unwrap_or_default();
        let mut cover_art =
            icarus_models::coverart::init::init_coverart_only_path(coverart_path.clone());
        let metadatapath = self.get_metadata_path(sourcepath).unwrap_or_default();

        let mut up = syncers::upload::Upload::default();
        let host = self.ica_action.retrieve_flag_value(&String::from("-h"));
        up.set_api(&host);

        cover_art.data = cover_art.to_data().unwrap();

        match self.get_songs(&metadatapath, sourcepath) {
            Ok(sngs) => {
                for song in sngs {
                    match Runtime::new()
                        .unwrap()
                        .block_on(up.upload_song_with_metadata(&token, &song, &cover_art))
                    {
                        Ok(o) => {
                            println!("Response: {o:?}");
                        }
                        Err(err) => {
                            println!("Error: {err:?}");
                        }
                    };
                }
            }
            Err(error) => {
                println!("Error: {error:?}");
            }
        }

        Ok(())
    }

    fn get_cover_art_path(&self, directory_path: &String) -> Result<String> {
        for entry in read_dir(std::path::Path::new(directory_path))? {
            let entry = entry?;

            let file_type = entry.file_type();
            let file_name = entry.file_name();

            println!("file type: {file_type:?}");
            println!("file name: {file_name:?}");

            if let En::ImageFile = self.find_file_extension(&file_name) {
                let directory_part = directory_path.clone();
                let fname = utilities::string::o_to_string(&file_name);
                let fullpath = format!("{}/{}", directory_part, &fname.unwrap());
                return Ok(fullpath);
            }
        }

        Ok(String::new())
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

                if extension == icarus_models::constants::file_extensions::audio::WAVEXTENSION[1..]
                    || extension
                        == icarus_models::constants::file_extensions::audio::FLACEXTENSION[1..]
                {
                    return En::SongFile;
                } else if extension == "json" {
                    return En::MetadataFile;
                } else if extension
                    == icarus_models::constants::file_extensions::image::JPGEXTENSION[1..]
                    || extension == "jpeg"
                    || extension == "png"
                {
                    return En::ImageFile;
                }
            }
            _ => {
                return En::Other;
            }
        }

        En::Other
    }

    fn get_metadata_path(&self, directory_path: &String) -> Result<String> {
        for entry in read_dir(std::path::Path::new(directory_path))? {
            let entry = entry?;

            let file_type = entry.file_type();
            let file_name = entry.file_name();

            println!("file type: {file_type:?}");
            println!("file name: {file_name:?}");

            if let En::MetadataFile = self.find_file_extension(&file_name) {
                let directory_part = directory_path.clone();
                let fname = utilities::string::o_to_string(&file_name);
                return Ok(format!("{}/{}", directory_part, &fname.unwrap()));
            }
        }

        Ok(String::new())
    }

    fn _check_for_no_confirm(&self) -> bool {
        for flag in self.ica_action.flags.iter() {
            if flag.flag == "-nc" {
                return true;
            }
        }
        false
    }
}
