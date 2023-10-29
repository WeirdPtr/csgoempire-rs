use crate::{
    api::{get_base_request, request::CSGOEmpireApiRequest, CSGOEmpireApi},
    models::{
        structs::api::response::info::UniqueInfoResponse, traits::endpoint::CSGOEmpireEndpoint,
    },
};
use reqwest::Method;
use std::collections::HashMap;

pub struct UniqueInfoEndpoint(HashMap<&'static str, String>);

impl CSGOEmpireEndpoint for UniqueInfoEndpoint {
    type Response = UniqueInfoResponse;

    const URL: &'static str = "/trading/user/inventory/unique-info";
    const METHOD: Method = Method::GET;

    fn headers_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.0
    }
}

impl UniqueInfoEndpoint {
    pub fn new(api_key: &'static str) -> Self {
        Self(get_base_request(api_key))
    }
}

impl From<UniqueInfoEndpoint> for HashMap<&'static str, String> {
    fn from(endpoint: UniqueInfoEndpoint) -> Self {
        endpoint.0
    }
}

impl From<UniqueInfoEndpoint> for CSGOEmpireApiRequest<UniqueInfoEndpoint> {
    fn from(endpoint: UniqueInfoEndpoint) -> Self {
        Self::new(endpoint)
    }
}

impl CSGOEmpireApi {
    pub fn unique_info(api_key: &'static str) -> CSGOEmpireApiRequest<UniqueInfoEndpoint> {
        UniqueInfoEndpoint::new(api_key).into()
    }
}
