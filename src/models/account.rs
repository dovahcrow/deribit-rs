use crate::models::{AssetKind, Currency, Direction, Request};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Default)]
pub struct GetPositionsRequest {
    pub currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<AssetKind>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetPositionsResponse {
    pub average_price: f64,
    pub average_price_usd: Option<f64>,
    pub delta: f64,
    pub direction: Direction,
    pub estimated_liquidation_price: Option<f64>,
    pub floating_profit_loss: f64,
    pub floating_profit_loss_usd: Option<f64>,
    pub index_price: f64,
    pub initial_margin: f64,
    pub instrument_name: String,
    pub kind: AssetKind,
    pub maintenance_margin: f64,
    pub mark_price: f64,
    pub open_orders_margin: f64,
    pub realized_profit_loss: f64,
    pub settlement_price: f64,
    pub size: f64,
    pub size_currency: Option<f64>,
    pub total_profit_loss: f64,
}

impl Request for GetPositionsRequest {
    const METHOD: &'static str = "private/get_positions";
    type Response = Vec<GetPositionsResponse>;
}
