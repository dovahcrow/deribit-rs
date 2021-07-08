use crate::models::{AdvanceOption, Direction, OrderState, OrderType, TimeInForce, Trigger};
use fehler::throw;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

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

#[derive(Debug, Clone)]
pub enum UserOrdersChannel {
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

impl<'de> Deserialize<'de> for UserOrdersChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        let segments: Vec<_> = s.split(".").collect();
        match segments.as_slice() {
            ["user", "orders", instrument_name, interval] => Ok(UserOrdersChannel::ByInstrument {
                instrument_name: instrument_name.to_string(),
                interval: interval.to_string(),
            }),
            ["user", "orders", kind, currency, interval] => Ok(UserOrdersChannel::ByKind {
                kind: kind.to_string(),
                currency: currency.to_string(),
                interval: interval.to_string(),
            }),
            _ => throw!(D::Error::invalid_value(
                Unexpected::Str(s),
                &"user.orders.{instrument_name}.{interval} or user.orders.{kind}.{currency}.{interval}"
            )),
        }
    }
}
impl Serialize for UserOrdersChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::fmt::Display for UserOrdersChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserOrdersChannel::ByInstrument {
                instrument_name,
                interval,
            } => write!(f, "user.orders.{}.{}", instrument_name, interval),
            UserOrdersChannel::ByKind {
                kind,
                currency,
                interval,
            } => {
                write!(f, "user.orders.{}.{}.{}", kind, currency, interval)
            }
        }
    }
}
