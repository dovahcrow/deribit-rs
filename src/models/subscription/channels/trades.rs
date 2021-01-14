use crate::models::{Direction, LiquidationType};
use fehler::throw;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

/// Attention: if this is used along with UserTrades,
/// please put this after UserTrades otherwise all UserTrades
/// will be deserialize to Trades since the Trades is a subset of UserTrades
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TradesData {
    pub amount: f64,
    pub direction: Direction,
    pub index_price: f64,
    pub instrument_name: String,
    pub iv: Option<f64>,
    pub liquidation: Option<LiquidationType>,
    pub price: f64,
    pub tick_direction: u64,
    pub timestamp: u64,
    pub trade_id: String,
    pub trade_seq: u64,
}

#[derive(Debug, Clone)]
pub enum TradesChannel {
    ByInstrument {
        instrument_name: String,
        interval: String,
    },
    ByKind {
        kind: String,
        currency: String,
        interval: String,
    },
}

impl<'de> Deserialize<'de> for TradesChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        let segments: Vec<_> = s.split(".").collect();
        match segments.as_slice() {
            ["trades", instrument_name, interval] => Ok(TradesChannel::ByInstrument {
                instrument_name: instrument_name.to_string(),
                interval: interval.to_string(),
            }),
            ["trades", kind, currency, interval] => Ok(TradesChannel::ByKind {
                kind: kind.to_string(),
                currency: currency.to_string(),
                interval: interval.to_string(),
            }),
            _ => throw!(D::Error::invalid_value(
                Unexpected::Str(s),
                &"trades.{instrument_name}.{interval} or trades.{kind}.{currency}.{interval}"
            )),
        }
    }
}
impl Serialize for TradesChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TradesChannel::ByInstrument {
                instrument_name,
                interval,
            } => serializer.serialize_str(&format!("trades.{}.{}", instrument_name, interval)),
            TradesChannel::ByKind {
                kind,
                currency,
                interval,
            } => serializer.serialize_str(&format!("trades.{}.{}.{}", kind, currency, interval)),
        }
    }
}
