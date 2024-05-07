use std::default::Default;

use crate::models;

use super::syncer_base;

#[derive(Clone, Debug)]
pub struct Delete {
    pub api: models::api::API,
}

impl Default for Delete {
    fn default() -> Self {
        Delete {
            api: models::api::API::default(),
        }
    }
}

impl Delete {
    pub fn delete_song(&self, token: &models::token::Token, song: &models::song::Song) {
        let url = syncer_base::Syncer::retrieve_url(&self.api, &song);
    }
}
