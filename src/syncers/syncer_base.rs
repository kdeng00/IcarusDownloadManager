use crate::models;

pub struct Syncer {
    ok: i32,
    unauthorized: i32,
    not_found: i32,
}

impl Syncer {
    pub fn retrieve_url(api: &models::api::API, song: &models::song::Song) -> String {
        let mut url: String = String::from(&api.url);
        url += &String::from("api/");
        url += &String::from(&api.version);
        url += &String::from("/");
        url += &String::from(&api.endpoint);
        url += &String::from("/");
        url += &song.id.unwrap().to_string();

        return url;
    }

    pub fn ok() -> i32 {
        return 200;
    }

    pub fn unauthorized() -> i32 {
        return 401;
    }

    pub fn not_found() -> i32 {
        return 404;
    }
}

pub enum Result {
    OK,
    UNAUTHORIZED,
    NOTFOUND,
}


