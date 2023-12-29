use crate::enums::trade::TradeStatus;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeStatusUpdateResponse {
    #[serde(rename = "type")]
    pub status_update_type: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub id: u64,
    pub total_value: u64,
    pub item_id: u64,
    pub item: Item,
    pub status: TradeStatus,
    pub status_message: String,
    pub tradeoffer_id: u64,
    pub created_at: String,
    pub metadata: Option<Metadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub market_name: String,
    pub market_value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub item_validation: ItemValidation,
    pub expires_at: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemValidation {
    #[serde(rename = "validItemDetected")]
    pub valid_item_detected: bool,
    #[serde(rename = "numWrongItemDetections")]
    pub num_wrong_item_detections: Option<u64>,
}
