use std::default::Default;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Api {
    pub url: String,
    pub endpoint: String,
    pub version: String,
}
