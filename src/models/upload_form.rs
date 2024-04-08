use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct UploadForm {
    pub url: String,
    pub filepath: String,
}