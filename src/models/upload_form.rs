use serde::{Deserialize, Serialize};

// mod models {

#[derive(Debug, Deserialize, Serialize)]
pub struct UploadForm {
    pub url: String,
    pub filepath: String,
}
// }
