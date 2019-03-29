use crate::models::{Currency, Direction, OrderState, OrderType, Role};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct UserTradesInstrumentNameIntervalRequest {
    pub instrument_name: String,
    pub interval: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserTradesInstrumentNameIntervalMessage {
    pub amount: f64,
    pub direction: Direction,
    pub fee: f64,
    pub fee_currency: Currency,
    pub index_price: f64,
    pub instrument_name: String,
    pub iv: Option<f64>,
    pub label: Option<String>,
    pub liquidity: Role,
    pub matching_id: Option<String>,
    pub order_id: String,
    pub order_type: OrderType,
    pub price: f64,
    pub self_trade: bool,
    pub state: OrderState,
    pub tick_direction: i64,
    pub timestamp: u128,
    pub trade_id: String,
    pub trade_seq: i64,
}
