use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Flags {
    pub flag: String,
    pub value: String,
}
