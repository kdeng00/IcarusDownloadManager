pub fn print_help() {
    let msg: String = String::from(
        r#"icd [Action] [flag]

        Actions
            download
            upload-meta
            retrieve
            delete

        Flags
        Required for all actions
            -u username
            -p password
            -h api host
            -ha auth api host

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

    println!("{msg}");
}
