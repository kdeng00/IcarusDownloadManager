use crate::models;

#[derive(Clone, Debug)]
pub struct Delete {
    pub api: models::api::API,
}

impl Delete {
    pub fn delete_song(&self, token: &models::token::Token, song: &models::song::Song) {}
}
