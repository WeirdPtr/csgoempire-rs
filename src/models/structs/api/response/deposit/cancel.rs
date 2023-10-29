use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelDepositResponse {
    #[serde(default)]
    pub success: bool,
    pub data: Option<HashMap<String, Data>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub success: bool,
    pub message: Option<String>,
}
