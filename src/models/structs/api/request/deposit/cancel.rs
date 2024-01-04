use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelDepositsRequest {
    pub ids: Vec<u64>,
}
