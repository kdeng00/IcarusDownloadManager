use std::default::Default;

#[derive(Clone, Debug, Default)]
pub struct Api {
    pub url: String,
    pub endpoint: String,
    pub version: String,
}
