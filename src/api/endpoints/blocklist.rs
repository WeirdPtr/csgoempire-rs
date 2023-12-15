use crate::{
    api::{get_base_request, request::CSGOEmpireApiRequest, CSGOEmpireApi},
    models::{
        structs::api::response::blocklist::BlockedUserResponse,
        traits::endpoint::CSGOEmpireEndpoint,
    },
};
use reqwest::Method;
use std::collections::HashMap;

pub struct BlocklistEndpoint(HashMap<&'static str, String>);

impl CSGOEmpireEndpoint for BlocklistEndpoint {
    type Response = Option<Vec<BlockedUserResponse>>;

    const URL: &'static str = "/trading/block-list";
    const METHOD: Method = Method::GET;

    fn headers_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.0
    }
}

impl BlocklistEndpoint {
    pub fn new<K>(api_key: K) -> Self
    where
        K: Into<String>,
    {
        Self(get_base_request(api_key))
    }
}

impl From<BlocklistEndpoint> for HashMap<&'static str, String> {
    fn from(endpoint: BlocklistEndpoint) -> Self {
        endpoint.0
    }
}

impl From<BlocklistEndpoint> for CSGOEmpireApiRequest<BlocklistEndpoint> {
    fn from(endpoint: BlocklistEndpoint) -> Self {
        Self::new(endpoint)
    }
}

impl CSGOEmpireApi {
    pub fn blocklist(api_key: impl Into<String>) -> CSGOEmpireApiRequest<BlocklistEndpoint> {
        BlocklistEndpoint::new(api_key).into()
    }
}
