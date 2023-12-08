use crate::models::{enums::CSGOEmpireApiRequestError, traits::endpoint::CSGOEmpireEndpoint};
use reqwest::StatusCode;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct CSGOEmpireApiRequest<E>
where
    E: CSGOEmpireEndpoint + Into<HashMap<&'static str, String>>,
{
    body: Option<String>,
    params: HashMap<&'static str, String>,
    shims: HashMap<&'static str, String>,
    endpoint: E,
}

impl<E> CSGOEmpireApiRequest<E>
where
    E: CSGOEmpireEndpoint + Into<HashMap<&'static str, String>>,
{
    pub fn new(endpoint: E) -> Self {
        Self {
            body: None,
            params: HashMap::new(),
            shims: HashMap::new(),
            endpoint,
        }
    }

    pub fn body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }

    pub fn param(mut self, key: &'static str, value: String) -> Self {
        self.params.insert(key, value);
        self
    }

    pub fn params(mut self, params: HashMap<&'static str, String>) -> Self {
        self.params = params;
        self
    }

    pub fn shim(mut self, key: &'static str, value: String) -> Self {
        self.shims.insert(key, value);
        self
    }

    pub fn shims(mut self, shims: HashMap<&'static str, String>) -> Self {
        self.shims = shims;
        self
    }

    pub fn url(&self) -> String {
        self.endpoint.url()
    }

    pub fn shimmed_url(&self) -> String {
        let mut url = self.url();

        for (key, value) in self.shims.iter() {
            if !value.is_empty() {
                url = url.replace(key, &value);
            }
        }

        url
    }

    pub fn inner(self) -> E {
        self.endpoint
    }

    pub fn inner_mut(&mut self) -> &mut E {
        &mut self.endpoint
    }

    pub async fn send(mut self) -> Result<E::Response, CSGOEmpireApiRequestError> {
        if let Some(shims) = self.endpoint.shims() {
            self.shims
                .extend(shims.into_iter().map(|(k, v)| (k.to_owned(), v.to_owned())));
        }

        let url = self.shimmed_url();

        if let Some(params) = self.endpoint.params() {
            self.params.extend(
                params
                    .into_iter()
                    .map(|(k, v)| (k.to_owned(), v.to_owned())),
            );
        }

        let headers = self.endpoint.headers();

        for header in E::REQUIRED_BASE_HEADERS
            .iter()
            .chain(E::REQUIRED_HEADERS.iter())
        {
            if !headers.contains_key(header) {
                return Err(CSGOEmpireApiRequestError::MissingHeader(header));
            }
        }

        for param in E::REQUIRED_PARAMS {
            if !self.params.contains_key(param) {
                return Err(CSGOEmpireApiRequestError::MissingParameter(param));
            }
        }

        let client = reqwest::Client::new();
        let mut request = client.request(E::METHOD, &url);

        for param in self.params.iter() {
            request = request.query(&[param]);
        }

        for (key, value) in headers.iter() {
            request = request.header(key.to_string(), value);
        }

        if let Some(body) = self.body {
            request = request.body(body);
        }

        let response = request
            .send()
            .await
            .map_err(|e| CSGOEmpireApiRequestError::ReqwestError(e))?;

        match response.status() {
            StatusCode::TOO_MANY_REQUESTS => return Err(CSGOEmpireApiRequestError::RateLimited),
            _ => {}
        }

        serde_json::from_str::<E::Response>(&response.text().await?)
            .map_err(|e| CSGOEmpireApiRequestError::Other(e.into()))
    }
}
