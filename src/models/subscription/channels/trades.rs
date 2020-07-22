use crate::models::{Direction, LiquidationType};
use serde::{Deserialize, Serialize};

/// Attention: if this is used along with UserTrades,
/// please put this after UserTrades otherwise all UserTrades
/// will be deserialize to Trades since they Trades is a subset of UserTrades
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TradesData {
    pub amount: f64,
    pub direction: Direction,
    pub index_price: f64,
    pub instrument_name: String,
    pub iv: Option<f64>,
    pub liquidation: Option<LiquidationType>,
    pub price: f64,
    pub tick_direction: u64,
    pub timestamp: u64,
    pub trade_id: String,
    pub trade_seq: u64,
}
