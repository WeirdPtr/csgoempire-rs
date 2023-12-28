use crate::{
    api::{get_base_request, request::CSGOEmpireApiRequest, CSGOEmpireApi},
    models::{
        structs::api::{request::deposit::bid::BidRequest, response::deposit::bid::BidResponse},
        traits::endpoint::CSGOEmpireEndpoint,
    },
};
use reqwest::Method;
use std::collections::HashMap;

pub struct BidEndpoint(
    HashMap<&'static str, String>,
    HashMap<&'static str, String>,
    BidRequest,
);

impl CSGOEmpireEndpoint for BidEndpoint {
    type Response = BidResponse;

    const URL: &'static str = "/trading/deposit/{deposit_id}/bid";
    const METHOD: Method = Method::POST;

    fn headers_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.0
    }

    fn shims_mut(&mut self) -> Option<&mut HashMap<&'static str, String>> {
        Some(&mut self.1)
    }
}

impl BidEndpoint {
    pub fn new(api_key: impl Into<String>, deposit_id: u64, bid_value: i64) -> Self {
        let mut shims = HashMap::new();

        shims.insert("{deposit_id}", deposit_id.to_string());

        Self(get_base_request(api_key), shims, BidRequest { bid_value })
    }

    pub fn deposit_id(mut self, deposit_id: u64) -> Self {
        self.1.insert("{deposit_id}", deposit_id.to_string());
        self
    }
}

impl From<BidEndpoint> for HashMap<&'static str, String> {
    fn from(endpoint: BidEndpoint) -> Self {
        endpoint.0
    }
}

impl From<BidEndpoint> for CSGOEmpireApiRequest<BidEndpoint> {
    fn from(endpoint: BidEndpoint) -> Self {
        let deposit_id = endpoint
            .1
            .get("{deposit_id}")
            .unwrap_or(&"".to_string())
            .to_owned();

        let body = serde_json::to_string(&endpoint.2).unwrap_or("".to_string());

        Self::new(endpoint)
            .shim("{deposit_id}", deposit_id)
            .body(body)
    }
}

impl CSGOEmpireApi {
    pub fn bid(
        api_key: impl Into<String>,
        deposit_id: u64,
        bid_value: i64,
    ) -> CSGOEmpireApiRequest<BidEndpoint> {
        BidEndpoint::new(api_key, deposit_id, bid_value).into()
    }
}
