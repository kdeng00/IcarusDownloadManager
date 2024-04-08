mod managers;
mod models;

use std::env;
use std::process;

fn exit_program(code: i32) {
    process::exit(code);
}

fn print_help() {
    let msg: String = String::from(
        r#"icd [Action] [flag]

        Actions
            download
            upload
            upload-meta
            retrieve
            delete

        Flags
        Required for all actions
            -u username
            -p password
            -h host

        Required for upload
            -s path of song
            -sd directory where to search for songs to upload (Optional)
            -sr directory where to recursively search for songs to upload (Optional)
            -nc will not prompt the user when uploading from a directory

        Required for upload with metadata
            -s path of song
            -t track number
            -m metadata filepath
            -ca coverart filepath
            -scma directory where songs, metadata, and cover art exists and will be uploaded (Optional)

        Required for download
            -b song id
            -d path to download song (Optional)

        Required for retrieving records
            -rt retrieve type (songs is only accepted)

        Required for deleting a song
            -D song id"#,
    );

    println!("{}", msg);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 0 {
        print_help();
        exit_program(-1);
    }

    let act_mgr = managers::action_managers::ActionManager {
        action: String::from(""),
        flags: Vec::new(),
        params: args,
        param_count: 2,
    };

    let chosen_act = act_mgr.retrieve_icarus_action();

    chosen_act.print_action_and_flags();

    let cmt_mgr = managers::commit_manager::CommitManager {
        action: String::from(""),
        flags: Vec::new(),
        params: Vec::new(),
        param_count: 1,
        ica_action: chosen_act,
    };

    cmt_mgr.commit_action();
}
