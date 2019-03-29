use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct BookInstrumentNameIntervalRequest {
    pub instrument_name: String,
    pub interval: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OrderBookDelta(pub String, pub f64, pub f64);

#[derive(Deserialize, Debug, Clone)]
pub struct BookInstrumentNameIntervalResponse {
    pub asks: Vec<OrderBookDelta>,
    pub bids: Vec<OrderBookDelta>,
    pub change_id: i64,
    pub instrument_name: String,
    pub prev_change_id: Option<i64>,
    pub timestamp: i64
}