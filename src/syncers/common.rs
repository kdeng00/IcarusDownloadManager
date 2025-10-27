use crate::models;

pub fn retrieve_url(api: &models::api::Api, with_id: bool, id: &uuid::Uuid) -> String {
    if with_id {
        retrieve_url_with_id(api, id)
    } else {
        retrieve_url_reg(api)
    }
}

fn retrieve_url_reg(api: &models::api::Api) -> String {
    format!("{}api/{}/{}/", api.url, api.version, api.endpoint)
}

fn retrieve_url_with_id(api: &models::api::Api, id: &uuid::Uuid) -> String {
    format!("{}api/{}/{}/{}", api.url, api.version, api.endpoint, id)
}

pub async fn auth_header(
    token: &icarus_models::token::AccessToken,
) -> Result<(http::HeaderName, http::HeaderValue), http::header::InvalidHeaderValue> {
    match http::HeaderValue::from_str(&token.bearer_token()) {
        Ok(auth_value) => Ok((reqwest::header::AUTHORIZATION, auth_value)),
        Err(err) => Err(err),
    }
}
