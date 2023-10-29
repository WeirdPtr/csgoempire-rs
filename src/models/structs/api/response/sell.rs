use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstantSellResponse {
    #[serde(default)]
    pub success: bool,
    pub auction_data: Option<AuctionData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuctionData {
    pub id: i64,
    pub app_id: i64,
    pub auction_highest_bid: i64,
    pub auction_highest_bidder: i64,
    pub auction_number_of_bids: i64,
    pub auction_ends_at: i64,
}
