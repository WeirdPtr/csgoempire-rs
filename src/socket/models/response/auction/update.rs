use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuctionUpdateResponse {
    pub id: u64,
    pub above_recommended_price: f64,
    pub auction_highest_bid: u64,
    pub auction_highest_bidder: u64,
    pub auction_number_of_bids: u64,
    pub auction_ends_at: u64,
}
