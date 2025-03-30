use crate::models;

pub fn retrieve_url(api: &models::api::API, with_id: bool, id: i32) -> String {
    if with_id {
        retrieve_url_with_id(&api, id)
    } else {
        retrieve_url_reg(&api)
    }
}

fn retrieve_url_reg(api: &models::api::API) -> String {
    let mut url: String = String::from(&api.url);
    url += &String::from("api/");
    url += &String::from(&api.version);
    url += &String::from("/");
    url += &String::from(&api.endpoint);
    url += &String::from("/");

    return url;
}

fn retrieve_url_with_id(api: &models::api::API, id: i32) -> String {
    let mut url: String = String::from(&api.url);
    url += &String::from("api/");
    url += &String::from(&api.version);
    url += &String::from("/");
    url += &String::from(&api.endpoint);
    url += &String::from("/");
    url += &id.to_string();

    return url;
}
