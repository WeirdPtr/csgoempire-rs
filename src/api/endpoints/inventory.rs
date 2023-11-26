use crate::{
    api::{get_base_request, request::CSGOEmpireApiRequest, CSGOEmpireApi},
    models::{
        enums::bool::CSGOEmpireBool, structs::api::response::inventory::InventoryResponse,
        traits::endpoint::CSGOEmpireEndpoint,
    },
};
use reqwest::Method;
use std::collections::HashMap;

pub struct InventoryEndpoint(HashMap<&'static str, String>, HashMap<&'static str, String>);

impl CSGOEmpireEndpoint for InventoryEndpoint {
    type Response = InventoryResponse;

    const URL: &'static str = "/trading/user/inventory";
    const METHOD: Method = Method::GET;

    fn headers_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.0
    }

    fn params_mut(&mut self) -> Option<&mut HashMap<&'static str, String>> {
        Some(&mut self.1)
    }
}

impl InventoryEndpoint {
    pub fn new<K>(api_key: K) -> Self
    where
        K: Into<String>,
    {
        let mut params = HashMap::new();

        params.insert("invalid", CSGOEmpireBool::No.to_string());

        Self(get_base_request(api_key), params)
    }

    pub fn invalid(&mut self, value: CSGOEmpireBool) -> &mut Self {
        self.1.insert("invalid", value.to_string());
        self
    }
}

impl From<InventoryEndpoint> for HashMap<&'static str, String> {
    fn from(endpoint: InventoryEndpoint) -> Self {
        endpoint.0
    }
}

impl From<InventoryEndpoint> for CSGOEmpireApiRequest<InventoryEndpoint> {
    fn from(endpoint: InventoryEndpoint) -> Self {
        Self::new(endpoint)
    }
}

impl CSGOEmpireApi {
    pub fn inventory(api_key: &'static str) -> CSGOEmpireApiRequest<InventoryEndpoint> {
        InventoryEndpoint::new(api_key).into()
    }
}
