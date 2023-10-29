use crate::{
    api::{get_base_request, request::CSGOEmpireApiRequest, CSGOEmpireApi},
    models::{
        structs::api::response::deposit::stats::DepositorStatsResponse,
        traits::endpoint::CSGOEmpireEndpoint,
    },
};
use reqwest::Method;
use std::collections::HashMap;

pub struct DepositorStatsEndpoint(HashMap<&'static str, String>, HashMap<&'static str, String>);

impl CSGOEmpireEndpoint for DepositorStatsEndpoint {
    type Response = DepositorStatsResponse;

    const URL: &'static str = "/trading/deposit/{depositor_id}/stats";
    const METHOD: Method = Method::GET;

    fn headers_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.0
    }

    fn shims_mut(&mut self) -> Option<&mut HashMap<&'static str, String>> {
        Some(&mut self.1)
    }
}

impl DepositorStatsEndpoint {
    pub fn new(api_key: &'static str, depositor_id: u64) -> Self {
        let mut shims = HashMap::new();

        shims.insert("depositor_id", depositor_id.to_string());
        Self(get_base_request(api_key), shims)
    }

    pub fn depositor_id(&mut self, depositor_id: u64) -> &mut Self {
        self.1.insert("depositor_id", depositor_id.to_string());
        self
    }
}

impl From<DepositorStatsEndpoint> for HashMap<&'static str, String> {
    fn from(endpoint: DepositorStatsEndpoint) -> Self {
        endpoint.0
    }
}

impl From<DepositorStatsEndpoint> for CSGOEmpireApiRequest<DepositorStatsEndpoint> {
    fn from(endpoint: DepositorStatsEndpoint) -> Self {
        Self::new(endpoint)
    }
}

impl CSGOEmpireApi {
    pub fn depositor_stats(
        api_key: &'static str,
        depositor_id: u64,
    ) -> CSGOEmpireApiRequest<DepositorStatsEndpoint> {
        DepositorStatsEndpoint::new(api_key, depositor_id).into()
    }
}
