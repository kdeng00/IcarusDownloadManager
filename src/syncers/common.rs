use crate::models;

pub fn retrieve_url(api: &models::api::Api, with_id: bool, id: &uuid::Uuid) -> String {
    if with_id {
        retrieve_url_with_id(api, id)
    } else {
        retrieve_url_reg(api)
    }
}

fn retrieve_url_reg(api: &models::api::Api) -> String {
    let url = format!("{}/api/{}/{}/", api.url, api.version, api.endpoint);

    url
}

fn retrieve_url_with_id(api: &models::api::Api, id: &uuid::Uuid) -> String {
    let url = format!("{}/api/{}/{}/{}", api.url, api.version, api.endpoint, id);

    url
}
