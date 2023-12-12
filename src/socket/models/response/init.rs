use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitResponse {
    pub authenticated: bool,
    #[serde(rename = "serverTime")]
    pub server_time: String,
    pub server: String,
    pub roles: Vec<Value>,
    pub helper_mod: bool,
    #[serde(rename = "mod")]
    pub moderator: bool,
    pub super_mod: bool,
    pub admin: bool,
    pub qa: bool,
    pub badge_text: Option<String>,
    pub badge_text_localized: Option<String>,
    pub badge_color: Option<String>,
}
