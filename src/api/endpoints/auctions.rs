use crate::{
    api::{get_base_request, request::CSGOEmpireApiRequest, CSGOEmpireApi},
    models::{
        structs::api::response::auctions::ActiveAuctionsResponse,
        traits::endpoint::CSGOEmpireEndpoint,
    },
};
use reqwest::Method;
use std::collections::HashMap;

pub struct ActiveAuctionsEndpoint(HashMap<&'static str, String>);

impl CSGOEmpireEndpoint for ActiveAuctionsEndpoint {
    type Response = ActiveAuctionsResponse;

    const URL: &'static str = "/trading/user/auctions";
    const METHOD: Method = Method::GET;

    fn headers_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.0
    }
}

impl ActiveAuctionsEndpoint {
    pub fn new(api_key: &'static str) -> Self {
        Self(get_base_request(api_key))
    }
}

impl From<ActiveAuctionsEndpoint> for HashMap<&'static str, String> {
    fn from(endpoint: ActiveAuctionsEndpoint) -> Self {
        endpoint.0
    }
}

impl From<ActiveAuctionsEndpoint> for CSGOEmpireApiRequest<ActiveAuctionsEndpoint> {
    fn from(endpoint: ActiveAuctionsEndpoint) -> Self {
        Self::new(endpoint)
    }
}

impl CSGOEmpireApi {
    pub fn active_auctions(api_key: &'static str) -> CSGOEmpireApiRequest<ActiveAuctionsEndpoint> {
        ActiveAuctionsEndpoint::new(api_key).into()
    }
}
