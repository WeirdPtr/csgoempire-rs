use crate::{
    api::{get_base_request, request::CSGOEmpireApiRequest, CSGOEmpireApi},
    models::{
        structs::api::{
            request::withdraw::create::CreateWithdrawalRequest,
            response::withdraw::create::CreateWithdrawalResponse,
        },
        traits::endpoint::CSGOEmpireEndpoint,
    },
};
use reqwest::Method;
use std::collections::HashMap;

pub struct CreateWithdrawalEndpoint(
    HashMap<&'static str, String>,
    HashMap<&'static str, String>,
    CreateWithdrawalRequest,
);

impl CSGOEmpireEndpoint for CreateWithdrawalEndpoint {
    type Response = CreateWithdrawalResponse;

    const URL: &'static str = "/trading/deposit/{deposit_id}/withdraw";
    const METHOD: Method = Method::POST;

    fn headers_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.0
    }

    fn shims_mut(&mut self) -> Option<&mut HashMap<&'static str, String>> {
        Some(&mut self.1)
    }
}

impl CreateWithdrawalEndpoint {
    pub fn new(api_key: &'static str, deposit_id: i64, coin_value: i64) -> Self {
        let mut shims = HashMap::new();

        shims.insert("{deposit_id}", deposit_id.to_string());

        Self(
            get_base_request(api_key),
            shims,
            CreateWithdrawalRequest { coin_value },
        )
    }

    pub fn deposit_id(mut self, deposit_id: i64) -> Self {
        self.1.insert("{deposit_id}", deposit_id.to_string());
        self
    }
}

impl From<CreateWithdrawalEndpoint> for HashMap<&'static str, String> {
    fn from(endpoint: CreateWithdrawalEndpoint) -> Self {
        endpoint.0
    }
}

impl From<CreateWithdrawalEndpoint> for CSGOEmpireApiRequest<CreateWithdrawalEndpoint> {
    fn from(endpoint: CreateWithdrawalEndpoint) -> Self {
        let deposit_id = endpoint
            .1
            .get("{deposit_id}")
            .unwrap_or(&"".to_string())
            .clone();
        let body = serde_json::to_string(&endpoint.2).unwrap_or_default();
        Self::new(endpoint)
            .shim("{deposit_id}", deposit_id)
            .body(body)
    }
}

impl CSGOEmpireApi {
    pub fn create_withdrawal(
        api_key: &'static str,
        deposit_id: i64,
        coin_value: i64,
    ) -> CSGOEmpireApiRequest<CreateWithdrawalEndpoint> {
        CreateWithdrawalEndpoint::new(api_key, deposit_id, coin_value).into()
    }
}
