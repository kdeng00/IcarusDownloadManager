use std::default::Default;

use crate::models;

pub struct Download{
    pub api: models::api::API,
}

impl Default for Download {
    fn default() -> Self {
        Download {
            api: models::api::API::default(),
        }
    }
}

impl Download {
    pub async fn download_song(&self, token: &models::token::Token) {
    }

    fn retrieve_url(&self) -> String {
        let mut url: String = String::new();

        return url;
    }
}