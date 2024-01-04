use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateWithdrawalRequest {
    pub coin_value: u64,
}
