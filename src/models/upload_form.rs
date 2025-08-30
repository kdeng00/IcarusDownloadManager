use std::default::Default;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct UploadForm {
    pub url: Option<String>,
    pub filepath: Option<String>,
}
