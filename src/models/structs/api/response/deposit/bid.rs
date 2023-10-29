use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BidResponse {
    #[serde(default)]
    pub success: bool,
    pub auction_data: Option<AuctionData>,
    pub invoice: Option<Invoice>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuctionData {
    pub id: i64,
    pub app_id: i64,
    pub auction_highest_bid: i64,
    pub auction_highest_bidder: i64,
    pub auction_number_of_bids: i64,
    pub auction_ends_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Invoice {
    pub user_id: i64,
    pub status: i64,
    pub processor_id: i64,
    pub currency_id: i64,
    pub amount_coins: i64,
    pub metadata: Metadata,
    pub ip: String,
    pub updated_at: i64,
    pub created_at: i64,
    pub id: i64,
    pub processor_ref: String,
    pub status_name: String,
    pub processor_name: String,
    pub currency_code: String,
    pub complete_at: Value,
    pub refunded_at: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub deposit_id: i64,
}
