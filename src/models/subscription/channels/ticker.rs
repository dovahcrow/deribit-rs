use crate::models::OrderState;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TickerData {
    pub ask_iv: Option<f64>,
    pub best_ask_amount: f64,
    pub best_ask_price: Option<f64>,
    pub best_bid_amount: f64,
    pub best_bid_price: Option<f64>,
    pub bid_iv: Option<f64>,
    pub current_funding: Option<f64>,
    pub estimated_delivery_price: Option<f64>,
    pub funding_8h: Option<f64>,
    pub greeks: Option<Greeks>,
    pub index_price: f64,
    pub instrument_name: String,
    pub interest_rate: Option<f64>,
    pub last_price: Option<f64>,
    pub mark_iv: Option<f64>,
    pub mark_price: f64,
    pub max_price: f64,
    pub min_price: f64,
    pub open_interest: f64,
    pub settlement_price: Option<f64>,
    pub state: OrderState,
    pub stats: Stats,
    pub timestamp: u64,
    pub underlying_index: Option<String>,
    pub underlying_price: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Greeks {
    pub delta: f64,
    pub gamma: f64,
    pub rho: f64,
    pub theta: f64,
    pub vega: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Stats {
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub volume: Option<f64>,
}
