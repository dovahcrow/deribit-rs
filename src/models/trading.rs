use crate::models::{AdvanceOption, Currency, Direction, OrderState, OrderType, Role, TimeInForce, Trigger};
use serde_derive::{Deserialize, Serialize};

pub type BuyRequest = TradeRequest;
pub type SellRequest = TradeRequest;

#[derive(Serialize, Debug, Clone)]
pub struct TradeRequest {
    pub instrument_name: String,
    pub amount: f64,
    pub r#type: OrderType,
    pub label: Option<String>,
    pub price: Option<f64>,
    pub time_in_force: TimeInForce,
    pub max_show: Option<f64>,
    pub post_only: bool,
    pub reduce_only: bool,
    pub stop_price: Option<f64>,
    pub trigger: Option<Trigger>,
    pub advanced: Option<AdvanceOption>,
}

impl TradeRequest {
    pub fn market(instrument_name: &str, amount: f64) -> TradeRequest {
        TradeRequest {
            instrument_name: instrument_name.into(),
            amount: amount,
            r#type: OrderType::Market,
            label: None,
            price: None,
            time_in_force: TimeInForce::GoodTilCancelled,
            max_show: None,
            post_only: false,
            reduce_only: false,
            stop_price: None,
            trigger: None,
            advanced: None,
        }
    }
}

pub type BuyResponse = TradeResponse;
pub type SellResponse = TradeResponse;

#[derive(Deserialize, Debug, Clone)]
pub struct TradeResponse {
    trades: Vec<Trade>,
    order: Order,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Trade {
    pub trade_seq: i64,
    pub trade_id: String,
    pub timestamp: u128,
    pub tick_direction: i64,
    pub state: OrderState,
    pub self_trade: bool,
    pub price: f64,
    pub order_type: OrderType,
    pub order_id: String,
    pub matching_id: Option<String>,
    pub liquidity: Role,
    pub label: Option<String>,
    pub instrument_name: String,
    pub index_price: f64,
    pub fee_currency: Currency,
    pub fee: f64,
    pub direction: Direction,
    pub amount: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Order {
    pub time_in_force: TimeInForce,
    pub reduce_only: bool,
    pub profit_loss: f64,
    pub price: String,
    pub post_only: bool,
    pub order_type: OrderType,
    pub order_state: OrderState,
    pub order_id: String,
    pub max_show: f64,
    pub last_update_timestamp: u128,
    pub label: Option<String>,
    pub is_liquidation: bool,
    pub instrument_name: String,
    pub filled_amount: f64,
    pub direction: Direction,
    pub creation_timestamp: u128,
    pub commission: f64,
    pub average_price: f64,
    pub api: bool,
    pub amount: f64,
}
