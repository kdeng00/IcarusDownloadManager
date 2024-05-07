use std::default::Default;

use crate::models;

pub struct Upload {
    pub api: models::api::API,
}

impl Default for Upload {
    fn default() -> Self {
        Upload {
            api: models::api::API::default(),
        }
    }
}