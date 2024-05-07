use std::default::Default;

use crate::models;


pub struct RetrieveRecords {
    pub api: models::api::API,
}


impl Default for RetrieveRecords {
    fn default() -> Self {
        RetrieveRecords {
            api: models::api::API::default(),
        }
    }
}