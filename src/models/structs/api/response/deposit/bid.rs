use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BidResponse {
    #[serde(default)]
    pub success: bool,
    pub message: Option<String>,
    pub auction_data: Option<AuctionData>,
    pub invoice: Option<Invoice>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuctionData {
    pub id: u64,
    pub app_id: Option<u64>,
    pub auction_highest_bid: u64,
    pub auction_highest_bidder: u64,
    pub auction_number_of_bids: u64,
    pub auction_ends_at: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Invoice {
    pub user_id: u64,
    pub status: u64,
    pub processor_id: u64,
    pub currency_id: u64,
    pub amount_coins: u64,
    pub metadata: Metadata,
    pub ip: String,
    pub updated_at: u64,
    pub created_at: u64,
    pub id: u64,
    pub processor_ref: String,
    pub status_name: String,
    pub processor_name: String,
    pub currency_code: String,
    pub complete_at: Value,
    pub refunded_at: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub deposit_id: u64,
}
