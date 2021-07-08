use crate::models::OrderState;
use fehler::throw;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

/// Attention: if this is used along with Tickers,
/// please put this after Tickers otherwise all Tickers
/// will be deserialize to Quotes since the Quotes is a subset of Tickers
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TickerData {
    pub ask_iv: Option<f64>,
    pub best_ask_amount: f64,
    pub best_ask_price: Option<f64>,
    pub best_bid_amount: f64,
    pub best_bid_price: Option<f64>,
    pub bid_iv: Option<f64>,
    pub current_funding: Option<f64>,
    pub delivery_price: Option<f64>,
    pub funding_8h: Option<f64>,
    pub greeks: Option<Greeks>,
    pub index_price: f64,
    pub instrument_name: String,
    pub interest_rate: Option<f64>,
    pub last_price: Option<f64>,
    pub mark_iv: Option<f64>,
    pub mark_price: f64,
    pub max_price: f64,
    pub min_price: f64,
    pub open_interest: f64,
    pub settlement_price: Option<f64>,
    pub state: OrderState,
    pub stats: Stats,
    pub timestamp: u64,
    pub underlying_index: Option<String>,
    pub underlying_price: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Greeks {
    pub delta: f64,
    pub gamma: f64,
    pub rho: f64,
    pub theta: f64,
    pub vega: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Stats {
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub volume: Option<f64>,
    pub price_change: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct TickerChannel(String, String);
impl<'de> Deserialize<'de> for TickerChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        let segments: Vec<_> = s.split(".").collect();
        match segments.as_slice() {
            ["ticker", instrument_name, interval] => Ok(TickerChannel(
                instrument_name.to_string(),
                interval.to_string(),
            )),
            _ => throw!(D::Error::invalid_value(
                Unexpected::Str(s),
                &"ticker.{instrument_name}.{interval}"
            )),
        }
    }
}
impl Serialize for TickerChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::fmt::Display for TickerChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ticker.{}.{}", self.0, self.1)
    }
}
