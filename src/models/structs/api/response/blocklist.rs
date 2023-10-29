use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockedUserResponse {
    pub id: i64,
    pub blocker_user_id: i64,
    pub blocked_user_id: i64,
    pub created_at: String,
    pub updated_at: String,
}
