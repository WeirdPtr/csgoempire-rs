use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InventoryResponse {
    #[serde(default)]
    pub success: bool,
    pub updated_at: Option<i64>,
    pub allow_update: Option<bool>,
    pub data: Option<Vec<Data>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub asset_id: i64,
    pub created_at: String,
    pub custom_price_percentage: Option<i64>,
    pub full_position: i64,
    pub icon_url: String,
    pub id: i64,
    pub invalid: Option<String>,
    pub is_commodity: bool,
    pub market_name: String,
    pub market_value: i64,
    pub name_color: String,
    pub position: Option<i64>,
    pub preview_id: Option<String>,
    pub price_is_unreliable: i64,
    pub stickers: Vec<Sticker>,
    pub tradable: bool,
    pub tradelock: bool,
    pub updated_at: String,
    pub wear: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sticker {
    pub sticker_id: i64,
    pub wear: Value,
    pub name: String,
    pub image: String,
}
