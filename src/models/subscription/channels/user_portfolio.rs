use crate::models::Currency;
use fehler::throw;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserPortfolioData {
    pub available_funds: f64,
    pub available_withdrawal_funds: f64,
    pub balance: f64,
    pub currency: Currency,
    pub delta_total: f64,
    pub equity: f64,
    pub estimated_liquidation_ratio: f64,
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
    pub options_value: f64,
    pub portfolio_margining_enabled: bool,
    pub projected_initial_margin: f64, //for portfolio margining users
    pub projected_delta_total: f64,
    pub projected_maintenance_margin: f64, //for portfolio margining users
    pub session_rpl: f64,
    pub session_upl: f64,
    pub total_pl: f64,
}

#[derive(Debug, Clone)]
pub struct UserPortfolioChannel(String);
impl<'de> Deserialize<'de> for UserPortfolioChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        let segments: Vec<_> = s.split(".").collect();
        match segments.as_slice() {
            ["user", "portfolio", currency] => Ok(UserPortfolioChannel(currency.to_string())),
            _ => throw!(D::Error::invalid_value(
                Unexpected::Str(s),
                &"user.portfolio.{currency}"
            )),
        }
    }
}
impl Serialize for UserPortfolioChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::fmt::Display for UserPortfolioChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "user.portfolio.{}", self.0)
    }
}
