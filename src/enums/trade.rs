use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i8)]
pub enum TradeStatus {
    Error = -1,
    Pending = 0,
    Received = 1,
    Processing = 2,
    Sending = 3,
    Confirming = 4,
    Sent = 5,
    Completed = 6,
    Declined = 7,
    Cancelled = 8,
    TimedOut = 9,
    Credited = 10,
}

impl Default for TradeStatus {
    fn default() -> Self {
        TradeStatus::Error
    }
}

impl From<i8> for TradeStatus {
    fn from(value: i8) -> Self {
        TradeStatus::from_i8(value)
    }
}

impl From<TradeStatus> for i8 {
    fn from(value: TradeStatus) -> Self {
        value.to_i8()
    }
}

impl TradeStatus {
    pub fn from_i8<V>(value: V) -> Self
    where
        V: Into<i8>,
    {
        match value.into() {
            -1 => TradeStatus::Error,
            0 => TradeStatus::Pending,
            1 => TradeStatus::Received,
            2 => TradeStatus::Processing,
            3 => TradeStatus::Sending,
            4 => TradeStatus::Confirming,
            5 => TradeStatus::Sent,
            6 => TradeStatus::Completed,
            7 => TradeStatus::Declined,
            8 => TradeStatus::Cancelled,
            9 => TradeStatus::TimedOut,
            10 => TradeStatus::Credited,
            _ => TradeStatus::default(),
        }
    }

    pub fn to_i8(&self) -> i8 {
        *self as i8
    }
}
