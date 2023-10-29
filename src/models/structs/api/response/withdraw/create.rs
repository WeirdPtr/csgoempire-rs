use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateWithdrawalResponse {
    #[serde(default)]
    pub success: bool,
    pub data: Option<Data>,
    pub invoice: Option<Invoice>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub id: i64,
    pub user_id: i64,
    pub item_id: Value,
    pub items: Vec<Item>,
    pub total_value: i64,
    pub security_code: String,
    pub tradeoffer_id: i64,
    pub trade_id: i64,
    pub status: i64,
    pub status_message: String,
    pub metadata: Metadata,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub app_id: i64,
    pub created_at: i64,
    pub custom_price_percentage: Value,
    pub icon_url: String,
    pub id: i64,
    pub img: String,
    pub is_commodity: bool,
    pub market_name: String,
    pub market_value: f64,
    pub name: String,
    pub name_color: String,
    pub paint_index: Value,
    pub preview_id: Value,
    pub price_is_unreliable: bool,
    pub tradable: bool,
    pub tradelock: bool,
    #[serde(rename = "type")]
    pub item_type: String,
    pub updated_at: Option<String>,
    pub wear: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub auction_highest_bid: Value,
    pub auction_highest_bidder: Value,
    pub auction_number_of_bids: i64,
    pub auction_ends_at: i64,
    pub auction_auto_withdraw_failed: Value,
    pub price_updated_at: Value,
    pub trade_url: Value,
    pub partner: Value,
    pub total_fee: Value,
    pub fee: Value,
    pub old_total_value: Value,
    pub item_position_in_inventory: i64,
    pub item_inspected: bool,
    pub steam_id: String,
    pub expires_at: Value,
    pub delivery_time: Value,
    #[serde(rename = "phishingScamDetected")]
    pub phishing_scam_detected: Value,
    pub item_validation: Value,
    pub possible_abuse_detected_at: Value,
    pub penalty: Value,
    pub service_name: String,
    pub service_invoice_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Invoice {
    pub user_id: i64,
    pub status: i64,
    pub processor_id: i64,
    pub currency_id: i64,
    pub amount_coins: i64,
    pub metadata: Metadata2,
    pub ip: String,
    pub updated_at: String,
    pub created_at: String,
    pub id: i64,
    pub processor_txid: String,
    pub user: User,
    pub status_name: String,
    pub processor_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata2 {
    pub deposit_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub steam_id: String,
    pub steam_id_v3: String,
    pub steam_name: String,
    pub avatar: String,
    pub profile_url: String,
    pub registration_timestamp: String,
    pub registration_ip: String,
    pub last_login: String,
    pub balance: i64,
    pub total_profit: i64,
    pub total_bet: i64,
    pub betback_total: i64,
    pub bet_threshold: i64,
    pub total_trades: i64,
    pub total_deposit: i64,
    pub total_withdraw: i64,
    pub withdraw_limit: i64,
    pub csgo_playtime: i64,
    pub last_csgo_playtime_cache: String,
    pub trade_url: String,
    pub trade_offer_token: String,
    pub ref_id: i64,
    pub total_referral_bet: i64,
    pub total_referral_commission: i64,
    pub ref_permission: i64,
    pub ref_earnings: i64,
    pub total_ref_earnings: i64,
    pub total_ref_count: i64,
    pub total_credit: i64,
    pub referral_code: Value,
    pub referral_amount: i64,
    pub muted_until: i64,
    pub mute_reason: String,
    pub admin: i64,
    pub super_mod: i64,
    #[serde(rename = "mod")]
    pub mod_field: i64,
    pub utm_campaign: String,
    pub country: String,
    pub is_vac_banned: i64,
    pub steam_level: i64,
    pub last_steam_level_cache: String,
    pub whitelisted: i64,
    pub total_tips_received: i64,
    pub total_tips_sent: i64,
    pub withdrawal_fee_owed: String,
    pub flags: i64,
    pub encrypted_fields: Vec<Value>,
    pub balances: Vec<Value>,
    pub kyc: Vec<Value>,
    pub steam_data: SteamData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SteamData {
    pub user_id: i64,
    pub timecreated: i64,
}
