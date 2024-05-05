use crate::models;

struct Syncer {}

impl Syncer {
    pub fn retrieve_url(api: &models::api::API, song: &models::song::Song) -> String {
        let mut url: String = String::from(&api.url);
        url += &String::from("api/");
        url += &String::from(&api.version);
        url += &String::from("/");
        url += &String::from(&api.endpoint);
        url += &String::from("/");
        url += &song.id.unwrap().to_string();

        return url;
    }
}
