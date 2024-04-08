use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct API {
    pub url: String,
    pub endpoint: String,
    pub version: String,
}
