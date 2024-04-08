use serde::{Deserialize, Serialize};

// mod models {
#[derive(Debug, Deserialize, Serialize)]
pub struct Flags {
    pub flag: String,
    pub value: String,
}
// }
