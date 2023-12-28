use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewItemResponse {
    pub auction_ends_at: Option<u64>,
    pub auction_highest_bid: Option<u64>,
    pub auction_highest_bidder: Option<u64>,
    pub auction_number_of_bids: i64,
    pub icon_url: String,
    pub is_commodity: bool,
    pub market_name: String,
    pub market_value: i64,
    pub name_color: String,
    pub preview_id: Option<String>,
    pub price_is_unreliable: bool,
    pub stickers: Vec<Sticker>,
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
pub struct Sticker {
    pub wear: Option<f64>,
    pub name: String,
    pub image: Option<String>,
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
    pub category: Option<String>,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    pub sub_type: Option<String>,
    pub rarity: Option<String>,
}
