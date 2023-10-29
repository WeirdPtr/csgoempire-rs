use crate::{
    api::{get_base_request, request::CSGOEmpireApiRequest, CSGOEmpireApi},
    models::{
        enums::bool::CSGOEmpireBool, structs::api::response::withdraw::items::ListedItemsResponse,
        traits::endpoint::CSGOEmpireEndpoint,
    },
};
use reqwest::Method;
use std::collections::HashMap;

pub struct ListedItemsEndpoint(HashMap<&'static str, String>, HashMap<&'static str, String>);

impl CSGOEmpireEndpoint for ListedItemsEndpoint {
    type Response = ListedItemsResponse;

    const URL: &'static str = "/trading/items";
    const METHOD: Method = Method::GET;

    const REQUIRED_PARAMS: &'static [&'static str] = &["per_page", "page"];

    fn headers_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.0
    }

    fn params_mut(&mut self) -> Option<&mut HashMap<&'static str, String>> {
        Some(&mut self.1)
    }
}

impl ListedItemsEndpoint {
    pub fn new(api_key: &'static str, per_page: u64, page: u64) -> Self {
        let mut params = HashMap::new();

        params.insert("per_page", per_page.to_string());
        params.insert("page", page.to_string());

        Self(get_base_request(api_key), params)
    }

    pub fn per_page(&mut self, per_page: u64) -> &mut Self {
        self.1.insert("per_page", per_page.to_string());
        self
    }

    pub fn page(&mut self, page: u64) -> &mut Self {
        self.1.insert("page", page.to_string());
        self
    }

    pub fn search<V>(&mut self, search: V) -> &mut Self
    where
        V: Into<String>,
    {
        self.1.insert("search", search.into());
        self
    }

    pub fn order<V>(&mut self, order: V) -> &mut Self
    where
        V: Into<String>,
    {
        self.1.insert("order", order.into());
        self
    }

    pub fn sort<V>(&mut self, sort: V) -> &mut Self
    where
        V: Into<String>,
    {
        self.1.insert("sort", sort.into());
        self
    }

    pub fn auction<V>(&mut self, auction: V) -> &mut Self
    where
        V: Into<String>,
    {
        self.1.insert("auction", auction.into());
        self
    }

    pub fn min_price<V>(&mut self, min_price: V) -> &mut Self
    where
        V: Into<String>,
    {
        self.1.insert("price_min", min_price.into());
        self
    }

    pub fn max_price<V>(&mut self, max_price: V) -> &mut Self
    where
        V: Into<String>,
    {
        self.1.insert("price_max", max_price.into());
        self
    }

    pub fn max_price_obove<V>(&mut self, max_price: V) -> &mut Self
    where
        V: Into<String>,
    {
        self.1.insert("price_max_above", max_price.into());
        self
    }

    pub fn min_wear(&mut self, min_wear: f32) -> &mut Self {
        self.1.insert("wear_min", min_wear.to_string());
        self
    }

    pub fn max_wear<V>(&mut self, max_wear: f32) -> &mut Self {
        self.1.insert("wear_max", max_wear.to_string());
        self
    }

    pub fn delivery_time_long_min<V>(&mut self, delivery_time: V) -> &mut Self
    where
        V: Into<String>,
    {
        self.1
            .insert("delivery_time_long_min", delivery_time.into());
        self
    }

    pub fn delivery_time_long_max<V>(&mut self, delivery_time: V) -> &mut Self
    where
        V: Into<String>,
    {
        self.1
            .insert("delivery_time_long_max", delivery_time.into());
        self
    }

    pub fn has_stickers(&mut self, has_stickers: CSGOEmpireBool) -> &mut Self {
        self.1.insert("has_stickers", has_stickers.into());
        self
    }

    pub fn is_commodity(&mut self, is_commodity: CSGOEmpireBool) -> &mut Self {
        self.1.insert("is_commodity", is_commodity.into());
        self
    }
}

impl From<ListedItemsEndpoint> for HashMap<&'static str, String> {
    fn from(endpoint: ListedItemsEndpoint) -> Self {
        endpoint.0
    }
}

impl From<ListedItemsEndpoint> for CSGOEmpireApiRequest<ListedItemsEndpoint> {
    fn from(endpoint: ListedItemsEndpoint) -> Self {
        let params = endpoint.1.clone();
        Self::new(endpoint).params(params)
    }
}

impl CSGOEmpireApi {
    pub fn listed_items(
        api_key: &'static str,
        per_page: u64,
        page: u64,
    ) -> CSGOEmpireApiRequest<ListedItemsEndpoint> {
        ListedItemsEndpoint::new(api_key, per_page, page).into()
    }
}
