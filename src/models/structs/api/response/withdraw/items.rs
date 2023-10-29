use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListedItemsResponse {
    #[serde(default)]
    pub success: bool,
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
    pub auction_ends_at: Option<i64>,
    pub auction_highest_bid: Option<i64>,
    pub auction_highest_bidder: Value,
    pub auction_number_of_bids: i64,
    pub custom_price_percentage: Option<String>,
    pub icon_url: String,
    pub is_commodity: bool,
    pub market_name: String,
    pub market_value: Option<i64>,
    pub name_color: String,
    pub preview_id: Option<String>,
    pub price_is_unreliable: bool,
    pub stickers: Vec<Value>,
    pub wear: Option<f64>,
    pub published_at: String,
    pub id: Option<i64>,
    pub depositor_stats: DepositorStats,
    pub above_recommended_price: Option<String>,
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
pub struct Link {
    pub url: Option<String>,
    pub label: String,
    pub active: bool,
}
