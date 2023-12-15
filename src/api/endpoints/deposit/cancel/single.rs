use crate::{
    api::{get_base_request, request::CSGOEmpireApiRequest, CSGOEmpireApi},
    models::{
        structs::api::response::settings::SettingsReponse, traits::endpoint::CSGOEmpireEndpoint,
    },
};
use reqwest::Method;
use std::collections::HashMap;

pub struct CancelDepositEndpoint(HashMap<&'static str, String>, HashMap<&'static str, String>);

impl CSGOEmpireEndpoint for CancelDepositEndpoint {
    type Response = SettingsReponse;

    const URL: &'static str = "/trading/deposit/{deposit_id}/cancel";
    const METHOD: Method = Method::POST;

    fn headers_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.0
    }

    fn shims_mut(&mut self) -> Option<&mut HashMap<&'static str, String>> {
        Some(&mut self.1)
    }
}

impl CancelDepositEndpoint {
    pub fn new(api_key: impl Into<String>, deposit_id: u64) -> Self {
        let mut shims = HashMap::new();

        shims.insert("{deposit_id}", deposit_id.to_string());

        Self(get_base_request(api_key), shims)
    }

    pub fn deposit_id(mut self, deposit_id: u64) -> Self {
        self.1.insert("{deposit_id}", deposit_id.to_string());
        self
    }
}

impl From<CancelDepositEndpoint> for HashMap<&'static str, String> {
    fn from(endpoint: CancelDepositEndpoint) -> Self {
        endpoint.0
    }
}

impl From<CancelDepositEndpoint> for CSGOEmpireApiRequest<CancelDepositEndpoint> {
    fn from(endpoint: CancelDepositEndpoint) -> Self {
        let deposit_id = endpoint
            .1
            .get("{deposit_id}")
            .unwrap_or(&"".to_string())
            .to_owned();
        Self::new(endpoint).shim("{deposit_id}", deposit_id)
    }
}

impl CSGOEmpireApi {
    pub fn cancel_deposit(
        api_key: impl Into<String>,
        deposit_id: u64,
    ) -> CSGOEmpireApiRequest<CancelDepositEndpoint> {
        CancelDepositEndpoint::new(api_key, deposit_id).into()
    }
}
