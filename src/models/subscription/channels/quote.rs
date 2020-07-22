use serde::{Deserialize, Serialize};

/// Attention: if this is used along with Tickers,
/// please put this after Tickers otherwise all Tickers
/// will be deserialize to Quotes since the Quotes is a subset of Tickers
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QuoteData {
    pub best_ask_amount: f64,
    pub best_ask_price: f64,
    pub best_bid_amount: f64,
    pub best_bid_price: f64,
    pub instrument_name: String,
    pub timestamp: u64,
}
