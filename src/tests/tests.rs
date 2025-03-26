// mod managers;
// mod tests;

// use i
// use crate::icarus_dm::managers;
// use icarus-dm;
// use managers;

mod tests {

    #[cfg(test)]
    fn test_song_prep_upload() {
        use crate::managers;

        let meta_path = String::from("tests/sample2_tracks/album.json");

        if !std::path::Path::new(&meta_path).exists() {
            assert!(false, "File does not exists: {:?}", meta_path);
        }

        match icarus_models::album::collection::parse_album(&meta_path) {
            Ok(album) => {
                for track in 1..3 {
                    let directory_path = std::path::Path::new(&meta_path);
                    let directory: String = directory_path.parent().unwrap().display().to_string();
                    // let track = 1;
                    let filename: String = if track < 10 {
                        String::from("track0")
                            + &track.to_string()
                            + icarus_models::constants::DEFAULTMUSICEXTENSION
                    } else {
                        String::from("track")
                            + &track.to_string()
                            + icarus_models::constants::DEFAULTMUSICEXTENSION
                    };
                    let disc = 1;
                    match managers::commit_manager::retrieve_song(
                        &album, track, disc, &directory, &filename,
                    ) {
                        Ok(song) => match song.to_data() {
                            Ok(_) => {
                                print!("Success");
                            }
                            Err(err) => {
                                assert!(false, "Error: {:?}", err.to_string());
                            }
                        },
                        Err(err) => {
                            assert!(false, "Error: {:?}", err.to_string());
                        }
                    }
                }
            }
            Err(err) => {
                assert!(false, "Error: {:?}", err.to_string());
            }
        }
    }
}
