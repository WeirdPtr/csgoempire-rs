use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetadataResponse {
    pub user: Option<User>,
    pub socket_token: Option<String>,
    pub socket_signature: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub steam_name: String,
    pub avatar: String,
    pub registration_timestamp: String,
    pub registration_ip: String,
    pub last_login: String,
    pub total_profit: i64,
    pub total_bet: i64,
    pub betback_total: i64,
    pub bet_threshold: i64,
    pub total_trades: i64,
    pub total_deposit: i64,
    pub total_withdraw: i64,
    pub withdraw_limit: i64,
    pub ref_id: i64,
    pub ref_earnings: i64,
    pub total_ref_earnings: i64,
    pub referral_code: Option<String>,
    pub muted_until: i64,
    pub mute_reason: Value,
    pub utm_campaign: Value,
    pub country: String,
    pub whitelisted: i64,
    pub total_tips_received: i64,
    pub total_tips_sent: i64,
    pub withdrawal_fee_owed: String,
    pub flags: i64,
    pub tos_version: String,
    pub balance: u64,
    pub ban: Value,
    pub steam_user_pivot: Option<SteamUserPivot>,
    pub balances: Vec<Value>,
    pub steam_id: String,
    pub steam_level: Value,
    pub last_steam_level_cache: Option<String>,
    pub trade_offer_token: Option<String>,
    pub trade_url: Option<String>,
    pub level: u64,
    pub xp: u64,
    pub user_hash: String,
    pub hashed_server_seed: String,
    pub roles: Vec<Value>,
    pub eligible_for_free_case: bool,
    pub extra_security_type: String,
    pub p2p_telegram_notifications_allowed: bool,
    pub p2p_telegram_notifications_enabled: bool,
    pub p2p_request_failed_trade_feedback: bool,
    pub p2p_filter_delivery_time_enabled: bool,
    pub unread_notifications: Vec<Value>,
    pub last_session: Option<LastSession>,
    pub steam_inventory_url: String,
    pub steam_api_key: Option<String>,
    pub has_crypto_deposit: bool,
    pub api_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LastSession {
    pub id: u64,
    pub user_id: u64,
    pub ip: String,
    pub expired: bool,
    pub created_at: String,
    pub updated_at: String,
    pub device_identifier: String,
    pub user_agent: String,
    pub hash: String,
    pub city: String,
    pub country: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SteamUserPivot {
    pub id: i64,
    pub steam_user_id: i64,
    pub user_id: i64,
    pub service_name: String,
    pub created_at: String,
    pub updated_at: String,
    pub steam_user: SteamUser,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SteamUser {
    pub id: i64,
    pub steam_id: String,
    pub name: String,
    pub avatar: String,
    pub profile_url: String,
    pub trade_url: String,
    pub trade_offer_token: String,
    pub level: Value,
    pub last_level_cache: Value,
    pub time_created: i64,
}
