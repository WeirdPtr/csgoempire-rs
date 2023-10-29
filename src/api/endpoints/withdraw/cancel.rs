use crate::{
    api::{get_base_request, request::CSGOEmpireApiRequest, CSGOEmpireApi},
    models::{
        structs::api::response::settings::SettingsReponse, traits::endpoint::CSGOEmpireEndpoint,
    },
};
use reqwest::Method;
use std::collections::HashMap;

pub struct CancelWithdrawalEndpoint(HashMap<&'static str, String>, HashMap<&'static str, String>);

impl CSGOEmpireEndpoint for CancelWithdrawalEndpoint {
    type Response = SettingsReponse;

    const URL: &'static str = "/trading/user/withdrawals/{withdrawal_id}";
    const METHOD: Method = Method::DELETE;

    const REQUIRED_HEADERS: &'static [&'static str] = &["Content-Type"];

    fn headers_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.0
    }

    fn shims_mut(&mut self) -> Option<&mut HashMap<&'static str, String>> {
        Some(&mut self.1)
    }
}

impl CancelWithdrawalEndpoint {
    pub fn new(api_key: &'static str, withdrawal_id: u64) -> Self {
        let mut headers = get_base_request(api_key);
        headers.insert("Content-Type", "application/json".to_string());

        let mut shims = HashMap::new();
        shims.insert("{withdrawal_id}", withdrawal_id.to_string());

        Self(headers, shims)
    }

    pub fn withdrawal_id(mut self, withdrawal_id: u64) -> Self {
        self.1.insert("{withdrawal_id}", withdrawal_id.to_string());
        self
    }
}

impl From<CancelWithdrawalEndpoint> for HashMap<&'static str, String> {
    fn from(endpoint: CancelWithdrawalEndpoint) -> Self {
        endpoint.0
    }
}

impl From<CancelWithdrawalEndpoint> for CSGOEmpireApiRequest<CancelWithdrawalEndpoint> {
    fn from(endpoint: CancelWithdrawalEndpoint) -> Self {
        let withdrawal_id = endpoint
            .1
            .get("{withdrawal_id}")
            .unwrap_or(&"".to_string())
            .to_owned();
        Self::new(endpoint).shim("{withdrawal_id}", withdrawal_id)
    }
}

impl CSGOEmpireApi {
    pub fn cancel_withdrawal(
        api_key: &'static str,
        withdrawal_id: u64,
    ) -> CSGOEmpireApiRequest<CancelWithdrawalEndpoint> {
        CancelWithdrawalEndpoint::new(api_key, withdrawal_id).into()
    }
}
