use crate::{
    api::{get_base_request, request::CSGOEmpireApiRequest, CSGOEmpireApi},
    models::{
        structs::api::{
            request::deposit::create::CreateDepositRequest, response::settings::SettingsReponse,
        },
        traits::endpoint::CSGOEmpireEndpoint,
    },
};
use reqwest::Method;
use std::collections::HashMap;

pub struct CreateDepositEndpoint(HashMap<&'static str, String>, Option<CreateDepositRequest>);

impl CSGOEmpireEndpoint for CreateDepositEndpoint {
    type Response = SettingsReponse;

    const URL: &'static str = "/trading/deposit";
    const METHOD: Method = Method::POST;

    const REQUIRED_HEADERS: &'static [&'static str] = &["Content-Type"];

    fn headers_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.0
    }
}

impl CreateDepositEndpoint {
    pub fn new(api_key: impl Into<String>, request: CreateDepositRequest) -> Self {
        let mut headers = get_base_request(api_key);
        headers.insert("Content-Type", "application/json".to_string());
        Self(headers, Some(request))
    }
}

impl From<CreateDepositEndpoint> for HashMap<&'static str, String> {
    fn from(endpoint: CreateDepositEndpoint) -> Self {
        endpoint.0
    }
}

impl From<CreateDepositEndpoint> for CSGOEmpireApiRequest<CreateDepositEndpoint> {
    fn from(endpoint: CreateDepositEndpoint) -> Self {
        let request = endpoint.1.clone().unwrap_or(CreateDepositRequest {
            ..Default::default()
        });

        Self::new(endpoint).body(serde_json::to_string(&request).unwrap_or_default())
    }
}

impl CSGOEmpireApi {
    pub fn create_deposit(
        api_key: impl Into<String>,
        request: CreateDepositRequest,
    ) -> CSGOEmpireApiRequest<CreateDepositEndpoint> {
        CreateDepositEndpoint::new(api_key, request).into()
    }
}
