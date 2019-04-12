use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Delta {
    New,
    Change,
    Delete,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OrderBookDelta(pub Delta, pub f64, pub f64);

#[derive(Deserialize, Debug, Clone)]
pub struct BookData {
    pub asks: Vec<OrderBookDelta>,
    pub bids: Vec<OrderBookDelta>,
    pub change_id: i64,
    pub instrument_name: String,
    pub prev_change_id: Option<i64>,
    pub timestamp: u64,
}
