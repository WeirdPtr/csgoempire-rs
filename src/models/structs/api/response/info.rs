use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniqueInfoResponse {
    #[serde(default)]
    pub success: bool,
    pub data: Option<Vec<Data>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub id: i64,
    pub asset_id: i64,
    pub wear: f64,
    pub stickers: Vec<Sticker>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sticker {
    pub slot: i64,
    pub sticker_id: i64,
    pub wear: Value,
    pub scale: Value,
    pub rotation: Value,
    pub tint_id: Value,
    pub name: String,
    pub image: String,
}
