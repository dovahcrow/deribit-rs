use crate::models::Currency;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Copy)]
pub struct UserPortfolioCurrencyRequest {
    pub currency: Currency,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserPortfolioCurrencyMessage {
    pub available_funds: f64,
    pub available_withdrawal_funds: f64,
    pub balance: f64,
    pub currency: Currency,
    pub delta_total: f64,
    pub equity: f64,
    pub futures_pl: f64,
    pub futures_session_rpl: f64,
    pub futures_session_upl: f64,
    pub initial_margin: f64,
    pub maintenance_margin: f64,
    pub margin_balance: f64,
    pub options_delta: f64,
    pub options_gamma: f64,
    pub options_pl: f64,
    pub options_session_rpl: f64,
    pub options_session_upl: f64,
    pub options_theta: f64,
    pub options_vega: f64,
    pub portfolio_margining_enabled: bool,
    pub projected_initial_margin: Option<f64>,     //for portfolio margining users
    pub projected_maintenance_margin: Option<f64>, //for portfolio margining users
    pub session_funding: f64,
    pub session_rpl: f64,
    pub session_upl: f64,
    pub total_pl: f64,
}
