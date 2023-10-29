use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionHistoryResponse {
    pub current_page: Option<i64>,
    pub data: Option<Vec<Data>>,
    pub first_page_url: Option<String>,
    pub from: Option<i64>,
    pub last_page: Option<i64>,
    pub last_page_url: Option<String>,
    pub links: Option<Vec<Link>>,
    pub next_page_url: Option<String>,
    pub path: Option<String>,
    pub per_page: Option<i64>,
    pub prev_page_url: Option<String>,
    pub to: Option<i64>,
    pub total: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub id: i64,
    pub key: String,
    pub type_field: String,
    pub balance: i64,
    pub delta: i64,
    pub balance_after: i64,
    pub timestamp: f64,
    pub timestamp_raw: i64,
    pub date: String,
    pub invoice_id: Value,
    pub data: SubData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubData {
    pub id: i64,
    pub processor_name: String,
    pub status: i64,
    pub status_name: String,
    pub metadata: Metadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub deposit_id: i64,
    pub payment_method: String,
    pub id: i64,
    pub auction_highest_bid: i64,
    pub auction_highest_bidder: i64,
    pub auction_number_of_bids: i64,
    pub auction_ends_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub url: Option<String>,
    pub label: String,
    pub active: bool,
}
