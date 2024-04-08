use serde::{Deserialize, Serialize};

// mod models {

#[derive(Debug, Deserialize, Serialize)]
pub struct API {
    pub url: String,
    pub endpoint: String,
    pub version: String,
}
// }
