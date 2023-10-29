use std::collections::HashMap;

use crate::constants::API_BASE_URL;

pub trait CSGOEmpireEndpoint {
    type Response: serde::de::DeserializeOwned;

    const BASE_URL: &'static str = API_BASE_URL;

    const URL: &'static str;
    const METHOD: reqwest::Method;

    const REQUIRED_PARAMS: &'static [&'static str] = &[];
    const REQUIRED_HEADERS: &'static [&'static str] = &[];
    const REQUIRED_BASE_HEADERS: &'static [&'static str] = &["Authorization", "User-Agent"];

    fn headers_mut(&mut self) -> &mut HashMap<&'static str, String>;

    fn headers(&mut self) -> &HashMap<&'static str, String> {
        self.headers_mut()
    }

    fn params_mut(&mut self) -> Option<&mut HashMap<&'static str, String>> {
        None
    }

    fn params(&mut self) -> Option<&HashMap<&'static str, String>> {
        let params = self.params_mut();

        if let Some(params) = params {
            Some(params)
        } else {
            None
        }
    }

    fn shims_mut(&mut self) -> Option<&mut HashMap<&'static str, String>> {
        None
    }

    fn shims(&mut self) -> Option<&HashMap<&'static str, String>> {
        let shims = self.shims_mut();

        if let Some(shims) = shims {
            Some(shims)
        } else {
            None
        }
    }

    fn url(&self) -> String {
        format!("{}{}", Self::BASE_URL, Self::URL)
    }
}
