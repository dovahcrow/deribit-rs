use crate::models::Direction;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TradesData {
    pub amount: f64,
    pub direction: Direction,
    pub index_price: f64,
    pub instrument_name: String,
    pub iv: Option<f64>,
    pub price: f64,
    pub tick_direction: u64,
    pub timestamp: u64,
    pub trade_id: String,
    pub trade_seq: u64,
}
