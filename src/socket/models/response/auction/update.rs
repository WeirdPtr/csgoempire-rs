use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuctionUpdateResponse {
    pub id: i64,
    pub above_recommended_price: f64,
    pub auction_highest_bid: i64,
    pub auction_highest_bidder: i64,
    pub auction_number_of_bids: i64,
    pub auction_ends_at: i64,
}