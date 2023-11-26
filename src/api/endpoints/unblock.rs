use crate::{
    api::{get_base_request, request::CSGOEmpireApiRequest, CSGOEmpireApi},
    models::{
        structs::api::response::block::BlockUserResponse, traits::endpoint::CSGOEmpireEndpoint,
    },
};
use reqwest::Method;
use std::collections::HashMap;

pub struct BlockUserEndpoint(HashMap<&'static str, String>, HashMap<&'static str, String>);

impl CSGOEmpireEndpoint for BlockUserEndpoint {
    type Response = BlockUserResponse;

    const URL: &'static str = "/trading/block-list/";
    const METHOD: Method = Method::DELETE;

    const REQUIRED_PARAMS: &'static [&'static str] = &["steamid"];

    fn headers_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.0
    }

    fn params_mut(&mut self) -> Option<&mut HashMap<&'static str, String>> {
        Some(&mut self.1)
    }
}

impl BlockUserEndpoint {
    pub fn new<K>(api_key: K) -> Self
    where
        K: Into<String>,
    {
        Self(get_base_request(api_key), HashMap::new())
    }

    pub fn steamid(&mut self, steamid: &'static str) -> &mut Self {
        self.1.insert("steamid", steamid.to_string());
        self
    }
}

impl From<BlockUserEndpoint> for HashMap<&'static str, String> {
    fn from(endpoint: BlockUserEndpoint) -> Self {
        endpoint.0
    }
}

impl From<BlockUserEndpoint> for CSGOEmpireApiRequest<BlockUserEndpoint> {
    fn from(endpoint: BlockUserEndpoint) -> Self {
        Self::new(endpoint)
    }
}

impl CSGOEmpireApi {
    pub fn unblock_user(api_key: &'static str) -> CSGOEmpireApiRequest<BlockUserEndpoint> {
        BlockUserEndpoint::new(api_key).into()
    }
}
