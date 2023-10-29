use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepositorStatsResponse {
    pub delivery_rate_recent: Option<i64>,
    pub delivery_rate_long: Option<i64>,
    pub delivery_time_minutes_recent: Option<u64>,
    pub delivery_time_minutes_long: Option<u64>,
    pub steam_level_min_range: Option<i64>,
    pub steam_level_max_range: Option<i64>,
    pub user_has_trade_notifications_enabled: Option<bool>,
    pub user_is_online: Option<Value>,
}
