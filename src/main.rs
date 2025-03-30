mod help;
mod managers;
mod models;
mod parsers;
mod syncers;
mod utilities;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len() as i32;

    if args_len == 1 {
        help::print_help();
        utilities::checks::exit_program(-1);
    }

    println!("Argument count: {}", args_len);

    let mut act_mgr = managers::action_managers::ActionManager::default();
    act_mgr.set_params(&args);
    act_mgr.initialize();

    let chosen_act = act_mgr.retrieve_icarus_action();
    chosen_act.print_action_and_flags();

    let mut cmt_mgr = managers::commit_manager::CommitManager {
        ica_action: chosen_act,
    };

    cmt_mgr.commit_action();
}

#[cfg(test)]
mod tests {
    use crate::managers;

    #[test]
    fn test_song_prep_upload() {
        let meta_path = String::from("tests/sample2_tracks/album.json");

        if !std::path::Path::new(&meta_path).exists() {
            assert!(false, "File does not exists: {:?}", meta_path);
        }

        match icarus_models::album::collection::parse_album(&meta_path) {
            Ok(album) => {
                for track in 1..3 {
                    let directory_path = std::path::Path::new(&meta_path);
                    let directory: String = directory_path.parent().unwrap().display().to_string();
                    let filename: String = if track < 10 {
                        String::from("track0")
                            + &track.to_string()
                            + icarus_models::constants::file_extensions::audio::DEFAULTMUSICEXTENSION
                    } else {
                        String::from("track")
                            + &track.to_string()
                            + icarus_models::constants::file_extensions::audio::DEFAULTMUSICEXTENSION
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

    #[test]
    fn test_minimum_action_and_args() {
        let args: Vec<String> = vec![
            "icarus-dm".to_string(),
            "download".to_string(),
            "-u".to_string(),
            "jamborie".to_string(),
            "-p".to_string(),
            "somethingsecr3t!".to_string(),
            "-h".to_string(),
            "https://music-server.com".to_string(),
        ];
        let arg_count = args.len() as i32;
        let minimum_arg_count = 8;
        assert!(arg_count >= minimum_arg_count);

        let mut act_mgr = managers::action_managers::ActionManager {
            action: String::new(),
            flags: Vec::new(),
            params: args,
            param_count: arg_count,
        };
        act_mgr.initialize();
        let ica = act_mgr.retrieve_icarus_action();
        let action = &ica.action;
        let flags = &ica.flags;

        assert!(action != "");
        assert!(flags.len() > 2);
        assert!(
            !(action != "download"
                && action != "upload"
                && action != "retrieve"
                && action != "upload-meta")
        );

        let mut found_count = 0;
        let mut flags_already_read = Vec::new();

        for flag in flags {
            if flags_already_read.contains(&flag.flag) {
                continue;
            }

            if flag.flag == "-u" {
                found_count += 1;
                flags_already_read.push(flag.flag.clone());
            } else if flag.flag == "-p" {
                found_count += 1;
                flags_already_read.push(flag.flag.clone());
            } else if flag.flag == "-h" {
                found_count += 1;
                flags_already_read.push(flag.flag.clone());
            }
        }

        let all_flags_found = found_count == 3;

        assert_eq!(found_count, 3, "Three flags are required: -u, -p, -h");
        assert!(all_flags_found, "All flags have not been found");
    }
}
