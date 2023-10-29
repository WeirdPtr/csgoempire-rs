use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SettingsReponse {
    #[serde(default)]
    pub success: bool,
    pub escrow_seconds: Option<i64>,
}
