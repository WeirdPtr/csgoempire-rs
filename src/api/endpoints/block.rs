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

    const URL: &'static str = "/trading/block-list/{steam_id}";
    const METHOD: Method = Method::POST;

    fn headers_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.0
    }

    fn shims_mut(&mut self) -> Option<&mut HashMap<&'static str, String>> {
        Some(&mut self.1)
    }
}

impl BlockUserEndpoint {
    pub fn new(api_key: impl Into<String>, steam_id: impl Into<String>) -> Self {
        let mut shims = HashMap::new();

        shims.insert("{steam_id}", steam_id.into());

        Self(get_base_request(api_key), shims)
    }

    pub fn steamid(&mut self, steam_id: &'static str) -> &mut Self {
        self.1.insert("{steam_id}", steam_id.to_string());
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
        let steam_id = endpoint
            .1
            .get("{steam_id}")
            .unwrap_or(&"".to_string())
            .to_owned();
        Self::new(endpoint).shim("{steam_id}", steam_id)
    }
}

impl CSGOEmpireApi {
    pub fn block_user(
        api_key: impl Into<String>,
        steam_id: impl Into<String>,
    ) -> CSGOEmpireApiRequest<BlockUserEndpoint> {
        BlockUserEndpoint::new(api_key, steam_id).into()
    }
}
