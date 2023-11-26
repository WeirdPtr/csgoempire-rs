use crate::{
    api::{get_base_request, request::CSGOEmpireApiRequest, CSGOEmpireApi},
    models::{
        structs::api::response::trades::ActiveTradesResponse, traits::endpoint::CSGOEmpireEndpoint,
    },
};
use reqwest::Method;
use std::collections::HashMap;

pub struct ActiceTradesEndpoint(HashMap<&'static str, String>);

impl CSGOEmpireEndpoint for ActiceTradesEndpoint {
    type Response = ActiveTradesResponse;

    const URL: &'static str = "/trading/user/trades";
    const METHOD: Method = Method::GET;

    fn headers_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.0
    }
}

impl ActiceTradesEndpoint {
    pub fn new<K>(api_key: K) -> Self
    where
        K: Into<String>,
    {
        Self(get_base_request(api_key))
    }
}

impl From<ActiceTradesEndpoint> for HashMap<&'static str, String> {
    fn from(endpoint: ActiceTradesEndpoint) -> Self {
        endpoint.0
    }
}

impl From<ActiceTradesEndpoint> for CSGOEmpireApiRequest<ActiceTradesEndpoint> {
    fn from(endpoint: ActiceTradesEndpoint) -> Self {
        Self::new(endpoint)
    }
}

impl CSGOEmpireApi {
    pub fn active_trades(api_key: &'static str) -> CSGOEmpireApiRequest<ActiceTradesEndpoint> {
        ActiceTradesEndpoint::new(api_key).into()
    }
}
