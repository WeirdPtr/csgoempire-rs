use crate::{ util::crate_user_agent};
use std::collections::HashMap;

pub(crate) mod endpoints;
pub mod request;

pub struct CSGOEmpireApi;

pub fn get_base_request(api_key: &'static str) -> HashMap<&'static str, String> {
    let mut request = HashMap::new();
    request.insert("User-Agent", crate_user_agent());
    request.insert("Authorization", format!("Bearer {}", api_key));
    request
}
