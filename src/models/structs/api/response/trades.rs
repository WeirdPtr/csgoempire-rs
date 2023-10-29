use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::enums::trade::TradeStatus;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActiveTradesResponse {
    #[serde(default)]
    pub success: bool,
    pub data: Option<Data>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub deposits: Vec<Deposit>,
    pub withdrawals: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Deposit {
    pub id: i64,
    pub service_name: String,
    pub service_invoice_id: i64,
    pub user_id: i64,
    pub item_id: i64,
    pub items: Vec<Item>,
    pub total_value: i64,
    pub security_code: String,
    pub tradeoffer_id: i64,
    pub trade_id: i64,
    pub status: i64,
    pub status_message: TradeStatus,
    pub metadata: Metadata,
    pub item_hash: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub asset_id: i64,
    pub created_at: String,
    pub custom_price_percentage: i64,
    pub full_position: i64,
    pub icon_url: String,
    pub id: i64,
    pub is_commodity: bool,
    pub market_name: String,
    pub market_value: f64,
    pub name_color: String,
    pub position: Value,
    pub preview_id: Value,
    pub price_is_unreliable: i64,
    pub tradable: bool,
    pub tradelock: bool,
    pub updated_at: String,
    pub wear: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub auction_highest_bid: Value,
    pub auction_highest_bidder: Value,
    pub auction_number_of_bids: i64,
    pub auction_ends_at: i64,
    pub auction_auto_withdraw_failed: Value,
    pub price_updated_at: Value,
    pub trade_url: Value,
    pub partner: Value,
    pub total_fee: Value,
    pub fee: Value,
    pub old_total_value: Value,
    pub item_position_in_inventory: Value,
    pub item_inspected: bool,
    pub expires_at: Value,
    pub delivery_time: Value,
    pub phishing_scam_detected: Value,
    pub item_validation: Value,
    pub penalty: Value,
}
