use std::default::Default;

use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct UploadForm {
    pub url: Option<String>,
    pub filepath: Option<String>,
}

impl Default for UploadForm {
    fn default() -> Self {
        UploadForm {
            url: None,
            filepath: None,
        }
    }
}