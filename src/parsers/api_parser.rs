
use crate::models;

pub struct APIParser {
    pub api: models::api::API,
    pub ica_act: models::icarus_action::IcarusAction,
}

impl APIParser {
    // pub fn init() -> APIParser {
    // }

    pub fn retrieve_api(&self) -> models::api::API {
        return models::api::API::default();
    }

    // TODO: Implement
    fn parse_api(&self) {
    }
}