use crate::models;

#[derive(Clone, Debug)]
pub struct Delete {
}

impl Delete {
    pub fn delete_song(&self, token: &models::token::Token, song: &models::song::Song) {
    }
}