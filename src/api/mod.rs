use crate::util::crate_user_agent;
use std::collections::HashMap;

pub(crate) mod endpoints;
pub mod request;

pub struct CSGOEmpireApi;

pub fn get_base_request<K>(api_key: K) -> HashMap<&'static str, String>
where
    K: Into<String>,
{
    let mut request = HashMap::new();
    request.insert("User-Agent", crate_user_agent());
    request.insert("Authorization", format!("Bearer {}", api_key.into()));
    request.insert("Accept", "application/json, text/plain, */*".to_string());
    request.insert("Content-Type", "application/json".to_string());
    request
}
