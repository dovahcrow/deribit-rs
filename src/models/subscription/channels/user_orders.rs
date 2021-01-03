use crate::models::{AdvanceOption, Direction, OrderState, OrderType, TimeInForce, Trigger};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserOrdersData {
    pub advanced: Option<AdvanceOption>,
    pub amount: f64,
    pub api: bool,
    pub average_price: f64,
    pub commission: f64,
    pub creation_timestamp: u64,
    pub direction: Direction,
    pub filled_amount: f64,
    pub implv: Option<f64>,
    pub instrument_name: String,
    pub is_liquidation: bool,
    pub label: String,
    pub last_update_timestamp: u64,
    pub max_show: f64,
    pub order_id: String,
    pub order_state: OrderState,
    pub order_type: OrderType,
    pub post_only: bool,
    pub price: f64,
    pub profit_loss: f64,
    pub reduce_only: bool,
    pub stop_price: Option<f64>,
    pub time_in_force: TimeInForce,
    pub trigger: Option<Trigger>,
    pub triggered: Option<bool>,
    pub usd: Option<f64>,
    pub replaced: bool, // TODO: Remove the Option when necessary
    pub web: bool,
}
