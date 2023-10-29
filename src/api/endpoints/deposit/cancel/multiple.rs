use crate::{
    api::{get_base_request, request::CSGOEmpireApiRequest, CSGOEmpireApi},
    models::{
        structs::api::{
            request::deposit::cancel::CancelDepositsRequest,
            response::deposit::cancel::CancelDepositResponse,
        },
        traits::endpoint::CSGOEmpireEndpoint,
    },
};
use reqwest::Method;
use std::collections::HashMap;

pub struct CancelDepositsEndpoint(HashMap<&'static str, String>, CancelDepositsRequest);

impl CSGOEmpireEndpoint for CancelDepositsEndpoint {
    type Response = CancelDepositResponse;

    const URL: &'static str = "/trading/deposit/cancel";
    const METHOD: Method = Method::POST;

    const REQUIRED_HEADERS: &'static [&'static str] = &["Content-Type"];

    fn headers_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.0
    }
}

impl CancelDepositsEndpoint {
    pub fn new(api_key: &'static str, ids: Vec<i64>) -> Self {
        let mut headers = get_base_request(api_key);
        headers.insert("Content-Type", "application/json".to_string());
        Self(headers, CancelDepositsRequest { ids })
    }

    pub fn ids(mut self, ids: Vec<i64>) -> Self {
        self.1.ids = ids;
        self
    }

    pub fn id(mut self, id: i64) -> Self {
        self.1.ids.push(id);
        self
    }
}

impl From<CancelDepositsEndpoint> for HashMap<&'static str, String> {
    fn from(endpoint: CancelDepositsEndpoint) -> Self {
        endpoint.0
    }
}

impl From<CancelDepositsEndpoint> for CSGOEmpireApiRequest<CancelDepositsEndpoint> {
    fn from(endpoint: CancelDepositsEndpoint) -> Self {
        let body = serde_json::to_string(&endpoint.1).unwrap_or_default();
        Self::new(endpoint).body(body)
    }
}

impl CSGOEmpireApi {
    pub fn cancel_deposits<I>(
        api_key: &'static str,
        ids: I,
    ) -> CSGOEmpireApiRequest<CancelDepositsEndpoint>
    where
        I: Into<Vec<i64>>,
    {
        CancelDepositsEndpoint::new(api_key, ids.into()).into()
    }
}
