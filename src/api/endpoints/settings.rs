use crate::{
    api::{get_base_request, request::CSGOEmpireApiRequest, CSGOEmpireApi},
    models::{
        structs::api::{request::settings::SettingsRequest, response::settings::SettingsReponse},
        traits::endpoint::CSGOEmpireEndpoint,
    },
};
use reqwest::Method;
use std::collections::HashMap;

pub struct SettingsEndpoint(HashMap<&'static str, String>, Option<SettingsRequest>);

impl CSGOEmpireEndpoint for SettingsEndpoint {
    type Response = SettingsReponse;

    const URL: &'static str = "/trading/user/settings";
    const METHOD: Method = Method::POST;

    const REQUIRED_HEADERS: &'static [&'static str] = &["Content-Type"];

    fn headers_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.0
    }
}

impl SettingsEndpoint {
    pub fn new<K>(api_key: K) -> Self
    where
        K: Into<String>,
    {
        let mut headers = get_base_request(api_key);
        headers.insert("Content-Type", "application/json".to_string());
        Self(headers, None)
    }

    pub fn trade_url<U>(&mut self, trade_url: U) -> &mut Self
    where
        U: Into<String>,
    {
        self.1 = Some(SettingsRequest {
            trade_url: trade_url.into(),
        });
        self
    }
}

impl From<SettingsEndpoint> for HashMap<&'static str, String> {
    fn from(endpoint: SettingsEndpoint) -> Self {
        endpoint.0
    }
}

impl From<SettingsEndpoint> for CSGOEmpireApiRequest<SettingsEndpoint> {
    fn from(endpoint: SettingsEndpoint) -> Self {
        let request = &endpoint.1.clone().unwrap_or(SettingsRequest {
            trade_url: "".to_owned(),
        });

        Self::new(endpoint).body(serde_json::to_string(request).unwrap_or_default())
    }
}

impl CSGOEmpireApi {
    pub fn settings(api_key: &'static str) -> CSGOEmpireApiRequest<SettingsEndpoint> {
        SettingsEndpoint::new(api_key).into()
    }
}
