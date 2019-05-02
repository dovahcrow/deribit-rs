use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct QuoteData {
    pub best_ask_amount: f64,
    pub best_ask_price: f64,
    pub best_bid_amount: f64,
    pub best_bid_price: f64,
    pub instrument_name: String,
    pub timestamp: u64,
}
