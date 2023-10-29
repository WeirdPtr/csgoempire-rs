use crate::{
    api::{get_base_request, request::CSGOEmpireApiRequest, CSGOEmpireApi},
    models::{
        structs::api::response::sell::InstantSellResponse, traits::endpoint::CSGOEmpireEndpoint,
    },
};
use reqwest::Method;
use std::collections::HashMap;

pub struct InstantSellEndpoint(HashMap<&'static str, String>, HashMap<&'static str, String>);

impl CSGOEmpireEndpoint for InstantSellEndpoint {
    type Response = InstantSellResponse;

    const URL: &'static str = "/trading/deposit/{deposit_id}/sell";
    const METHOD: Method = Method::POST;

    fn headers_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.0
    }

    fn shims_mut(&mut self) -> Option<&mut HashMap<&'static str, String>> {
        Some(&mut self.1)
    }
}

impl InstantSellEndpoint {
    pub fn new(api_key: &'static str, deposit_id: u64) -> Self {
        let mut shims = HashMap::new();

        shims.insert("{deposit_id}", deposit_id.to_string());

        Self(get_base_request(api_key), shims)
    }

    pub fn deposit_id(&mut self, deposit_id: u64) -> &mut Self {
        self.1.insert("{deposit_id}", deposit_id.to_string());
        self
    }
}

impl From<InstantSellEndpoint> for HashMap<&'static str, String> {
    fn from(endpoint: InstantSellEndpoint) -> Self {
        endpoint.0
    }
}

impl From<InstantSellEndpoint> for CSGOEmpireApiRequest<InstantSellEndpoint> {
    fn from(endpoint: InstantSellEndpoint) -> Self {
        let deposit_id = endpoint
            .1
            .get("{deposit_id}")
            .unwrap_or(&"".to_string())
            .to_owned();
        Self::new(endpoint).shim("{deposit_id}", deposit_id)
    }
}

impl CSGOEmpireApi {
    pub fn instant_sell(
        api_key: &'static str,
        deposit_id: u64,
    ) -> CSGOEmpireApiRequest<InstantSellEndpoint> {
        InstantSellEndpoint::new(api_key, deposit_id).into()
    }
}
