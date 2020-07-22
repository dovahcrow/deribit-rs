use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Delta {
    New,
    Change,
    Delete,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OrderBookDelta(pub Delta, pub f64, pub f64);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BookData {
    pub asks: Vec<OrderBookDelta>,
    pub bids: Vec<OrderBookDelta>,
    pub change_id: i64,
    pub instrument_name: String,
    pub prev_change_id: Option<i64>,
    pub timestamp: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GroupedBookData {
    pub asks: Vec<(f64, f64)>,
    pub bids: Vec<(f64, f64)>,
    pub change_id: i64,
    pub instrument_name: String,
    pub prev_change_id: Option<i64>,
    pub timestamp: u64,
}
