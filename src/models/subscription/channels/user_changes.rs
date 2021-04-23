use crate::models::{
    AdvanceOption, AssetKind, Currency, Direction, LiquidationType, LiquidityType, OrderState,
    OrderType, TimeInForce, Trigger,
};

use fehler::throw;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserChangesData {
    pub trades: Vec<UserTradesData>,
    pub positions: Vec<UserPositionsData>,
    pub orders: Vec<UserOrdersData>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserTradesData {
    pub amount: f64,
    pub direction: Direction,
    pub fee: f64,
    pub fee_currency: Currency,
    pub index_price: f64,
    pub instrument_name: String,
    pub iv: Option<f64>,
    pub label: Option<String>,
    pub liquidity: LiquidityType,
    pub liquidation: Option<LiquidationType>,
    pub mark_price: Option<f64>,
    pub matching_id: Option<String>,
    pub order_id: String,
    pub order_type: OrderType,
    pub original_order_type: Option<String>,
    pub price: f64,
    pub profit_loss: f64,
    pub self_trade: bool,
    pub state: OrderState,
    pub tick_direction: i64,
    pub timestamp: u64,
    pub trade_id: String,
    pub trade_seq: i64,
    pub reduce_only: bool,
    pub post_only: bool,
}

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
    pub replaced: bool,
    pub web: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserPositionsData {
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
    pub leverage: f64,
    pub maintenance_margin: f64,
    pub mark_price: f64,
    pub open_orders_margin: f64,
    pub realized_funding: f64,
    pub realized_profit_loss: f64,
    pub settlement_price: f64,
    pub size: f64,
    pub size_currency: Option<f64>,
    pub total_profit_loss: f64,
}

#[derive(Debug, Clone)]
pub enum UserChangesChannel {
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

impl<'de> Deserialize<'de> for UserChangesChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        let segments: Vec<_> = s.split(".").collect();
        match segments.as_slice() {
            ["user", "changes", instrument_name, interval] => Ok(UserChangesChannel::ByInstrument {
                instrument_name: instrument_name.to_string(),
                interval: interval.to_string(),
            }),
            ["user", "changes", kind, currency, interval] => Ok(UserChangesChannel::ByKind {
                kind: kind.to_string(),
                currency: currency.to_string(),
                interval: interval.to_string(),
            }),
            _ => throw!(D::Error::invalid_value(
                Unexpected::Str(s),
                &"user.changes.{instrument_name}.{interval} or user.changes.{kind}.{currency}.{interval}"
            )),
        }
    }
}
impl Serialize for UserChangesChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            UserChangesChannel::ByInstrument {
                instrument_name,
                interval,
            } => {
                serializer.serialize_str(&format!("user.changes.{}.{}", instrument_name, interval))
            }
            UserChangesChannel::ByKind {
                kind,
                currency,
                interval,
            } => serializer
                .serialize_str(&format!("user.changes.{}.{}.{}", kind, currency, interval)),
        }
    }
}
