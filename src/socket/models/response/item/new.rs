use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewItemResponse {
    pub auction_ends_at: i64,
    pub auction_highest_bid: Option<u64>,
    pub auction_highest_bidder: Option<u64>,
    pub auction_number_of_bids: i64,
    pub icon_url: String,
    pub is_commodity: bool,
    pub market_name: String,
    pub market_value: i64,
    pub name_color: String,
    pub preview_id: Value,
    pub price_is_unreliable: bool,
    pub stickers: Vec<Value>,
    pub suggested_price: i64,
    pub wear: Option<f64>,
    pub published_at: String,
    pub id: i64,
    pub depositor_stats: DepositorStats,
    pub above_recommended_price: f64,
    pub purchase_price: i64,
    pub item_search: ItemSearch,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepositorStats {
    pub delivery_rate_recent: Option<f64>,
    pub delivery_rate_long: Option<f64>,
    pub delivery_time_minutes_recent: Option<i64>,
    pub delivery_time_minutes_long: Option<i64>,
    pub steam_level_min_range: Option<i64>,
    pub steam_level_max_range: Option<i64>,
    pub user_has_trade_notifications_enabled: bool,
    pub user_online_status: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemSearch {
    pub category: String,
    #[serde(rename = "type")]
    pub item_type: Value,
    pub sub_type: Value,
    pub rarity: String,
}