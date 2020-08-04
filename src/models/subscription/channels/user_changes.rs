use serde::{Deserialize, Serialize};
use crate::models::subscription::{UserOrdersData, UserTradesData};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserChangesData {
    instrument_name: String,
    orders: Vec<UserOrdersData>,
    positions: Vec<Position>,
    trades: Vec<UserTradesData>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Position {
    average_price: f64,
    average_price_usd: Option<f64>,
    delta: f64,
    direction: String,
    estimated_liquidation_price: Option<f64>,
    floating_profit_loss: f64,
    floating_profit_loss_usd: f64,
    gamma: Option<f64>,
    index_price: f64,
    initial_margin: f64,
    instrument_name: String,
    kind: String,
    leverage: i64,
    maintenance_margin: f64,
    mark_price: f64,
    open_orders_margin: f64,
    realized_funding: Option<f64>,
    realized_profit_loss: f64,
    settlement_price: f64,
    size: f64,
    size_currency: Option<f64>,
    theta: Option<f64>,
    total_profit_loss: f64,
    vega: Option<f64>,
}