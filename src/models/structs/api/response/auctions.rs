use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActiveAuctionsResponse {
    #[serde(default)]
    pub success: bool,
    pub active_auctions: Option<Vec<ActiveAuction>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActiveAuction {
    pub auction_ends_at: i64,
    pub auction_highest_bid: i64,
    pub auction_highest_bidder: i64,
    pub auction_number_of_bids: i64,
    pub custom_price_percentage: i64,
    pub icon_url: String,
    pub is_commodity: bool,
    pub market_name: String,
    pub market_value: i64,
    pub name_color: String,
    pub preview_id: Value,
    pub price_is_unreliable: bool,
    pub stickers: Vec<Value>,
    pub wear: Value,
    pub published_at: String,
    pub id: i64,
    pub depositor_stats: DepositorStats,
    pub above_recommended_price: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepositorStats {
    pub delivery_rate_recent: f64,
    pub delivery_rate_long: f64,
    pub delivery_time_minutes_recent: i64,
    pub delivery_time_minutes_long: i64,
    pub steam_level_min_range: i64,
    pub steam_level_max_range: i64,
    pub user_has_trade_notifications_enabled: bool,
    pub user_is_online: Value,
}
