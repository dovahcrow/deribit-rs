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

#[derive(Serialize, Debug, Clone, Default)]
pub struct GetAccountSummaryRequest {
    currency: Currency,
    extended: bool,
}
impl GetAccountSummaryRequest {
    pub fn abridged(currency: Currency) -> Self {
        Self {
            currency,
            ..Default::default()
        }
    }
    pub fn extended(currency: Currency) -> Self {
        Self {
            currency,
            extended: true,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetAccountSummaryResponse {
    id: Option<u64>,
    system_name: Option<String>,
    username: Option<String>,
    email: Option<String>,
    tfa_enabled: Option<bool>,
    portfolio_margin_enabled: Option<bool>,
    deposit_address: String,
    currency: Currency,
    r#type: Option<String>,
    session_funding: f64,

    maintenance_margin: f64,
    projected_initial_margin: Option<f64>,
    projected_maintenance_margin: Option<f64>,
    initial_margin: f64,
    margin_balance: f64,
    balance: f64,
    equity: f64,
    available_withdrawal_funds: f64,
    available_funds: f64,

    futures_session_upl: f64,
    futures_session_rpl: f64,
    futures_pl: f64,

    options_gamma: f64,
    options_vega: f64,
    options_theta: f64,
    options_delta: f64,
    options_session_upl: f64,
    options_session_rpl: f64,
    options_pl: f64,

    delta_total: f64,

    session_upl: f64,
    session_rpl: f64,
    total_pl: f64,
}

impl Request for GetAccountSummaryRequest {
    const METHOD: &'static str = "private/get_account_summary";
    type Response = GetAccountSummaryResponse;
}
