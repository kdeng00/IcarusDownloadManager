use std::default::Default;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Flags {
    pub flag: String,
    pub value: String,
}
