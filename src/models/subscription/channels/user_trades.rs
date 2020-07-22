use crate::models::{Currency, Direction, LiquidationType, LiquidityType, OrderState, OrderType};
use serde::{Deserialize, Serialize};

// This is for
// user.trades.{kind}.{currency}.{interval}.rs
// user.trades.{instrument_name}.{interval}
/// Attention: if this is used along with UserTrades,
/// please put this after UserTrades otherwise all UserTrades
/// will be deserialize to Trades since they Trades is a subset of UserTrades
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserTradesData {
    pub amount: f64,
    pub direction: Direction,
    pub fee: f64,
    pub fee_currency: Currency,
    pub index_price: f64,
    pub instrument_name: String,
    pub iv: Option<f64>,
    pub label: Option<String>,
    pub liquidity: LiquidityType,
    pub liquidation: Option<LiquidationType>,
    pub mark_price: Option<f64>,
    pub matching_id: Option<String>,
    pub order_id: String,
    pub order_type: OrderType,
    pub original_order_type: Option<String>,
    pub price: f64,
    pub profit_loss: f64,
    pub self_trade: bool,
    pub state: OrderState,
    pub tick_direction: i64,
    pub timestamp: u64,
    pub trade_id: String,
    pub trade_seq: i64,
    pub reduce_only: bool,
    pub post_only: bool,
}
