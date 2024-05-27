// use crate::models;

pub struct Syncer {
    // pub api: models::api::API,
    ok: i32,
    unauthorized: i32,
    not_found: i32,
}

impl Syncer {
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
