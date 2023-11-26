use crate::{
    api::{get_base_request, request::CSGOEmpireApiRequest, CSGOEmpireApi},
    models::{
        structs::api::response::metadata::MetadataResponse, traits::endpoint::CSGOEmpireEndpoint,
    },
};
use reqwest::Method;
use std::collections::HashMap;

pub struct MetadataEndpoint(HashMap<&'static str, String>);

impl CSGOEmpireEndpoint for MetadataEndpoint {
    type Response = MetadataResponse;

    const URL: &'static str = "/metadata/socket";
    const METHOD: Method = Method::GET;

    fn headers_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.0
    }
}

impl MetadataEndpoint {
    pub fn new<K>(api_key: K) -> Self
    where
        K: Into<String>,
    {
        Self(get_base_request(api_key))
    }
}

impl From<MetadataEndpoint> for HashMap<&'static str, String> {
    fn from(endpoint: MetadataEndpoint) -> Self {
        endpoint.0
    }
}

impl From<MetadataEndpoint> for CSGOEmpireApiRequest<MetadataEndpoint> {
    fn from(endpoint: MetadataEndpoint) -> Self {
        Self::new(endpoint)
    }
}

impl CSGOEmpireApi {
    pub fn metadata(api_key: &'static str) -> CSGOEmpireApiRequest<MetadataEndpoint> {
        MetadataEndpoint::new(api_key).into()
    }
}
