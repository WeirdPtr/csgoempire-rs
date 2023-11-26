use crate::{
    api::{get_base_request, request::CSGOEmpireApiRequest, CSGOEmpireApi},
    models::{
        structs::api::response::transactions::TransactionHistoryResponse,
        traits::endpoint::CSGOEmpireEndpoint,
    },
};
use reqwest::Method;
use std::collections::HashMap;

pub struct TransactionHistoryEndpoint(HashMap<&'static str, String>, HashMap<&'static str, String>);

impl CSGOEmpireEndpoint for TransactionHistoryEndpoint {
    type Response = TransactionHistoryResponse;

    const URL: &'static str = "/user/transactions";
    const METHOD: Method = Method::GET;

    const REQUIRED_PARAMS: &'static [&'static str] = &["page"];

    fn headers_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.0
    }

    fn params_mut(&mut self) -> Option<&mut HashMap<&'static str, String>> {
        Some(&mut self.1)
    }
}

impl TransactionHistoryEndpoint {
    pub fn new<K>(api_key: K) -> Self
    where
        K: Into<String>,
    {
        Self(get_base_request(api_key), HashMap::new())
    }

    pub fn page(&mut self, page: u64) -> &mut Self {
        self.1.insert("page", page.to_string());
        self
    }
}

impl From<TransactionHistoryEndpoint> for HashMap<&'static str, String> {
    fn from(endpoint: TransactionHistoryEndpoint) -> Self {
        endpoint.0
    }
}

impl From<TransactionHistoryEndpoint> for CSGOEmpireApiRequest<TransactionHistoryEndpoint> {
    fn from(endpoint: TransactionHistoryEndpoint) -> Self {
        Self::new(endpoint)
    }
}

impl CSGOEmpireApi {
    pub fn transaction_history(
        api_key: &'static str,
    ) -> CSGOEmpireApiRequest<TransactionHistoryEndpoint> {
        TransactionHistoryEndpoint::new(api_key).into()
    }
}
