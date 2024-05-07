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