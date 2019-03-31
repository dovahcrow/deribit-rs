mod account;
mod authentication;
mod message;
mod subscription;
mod support;
mod trading;

pub use account::{GetPositionsRequest, GetPositionsResponse};
pub use authentication::{AuthRequest, AuthResponse, GrantType};
pub use message::{JSONRPCRequest, JSONRPCResponse, SubscriptionData, SubscriptionMessage, WSMessage};
use serde_derive::{Deserialize, Serialize};
pub use subscription::channel::{Delta, BookInstrumentNameIntervalMessage, BookInstrumentNameIntervalRequest, OrderBookDelta};
pub use subscription::channel::{UserPortfolioCurrencyMessage, UserPortfolioCurrencyRequest};
pub use subscription::channel::{UserTradesInstrumentNameIntervalMessage, UserTradesInstrumentNameIntervalRequest};
pub use subscription::{SubscribeRequest, SubscribeResponse};
pub use support::{GetTimeResponse, HelloRequest, HelloResponse, TestRequest, TestResponse};
pub use trading::{BuyRequest, BuyResponse, Order, SellRequest, SellResponse, Trade, TradeRequest, TradeResponse, CancelResponse, CancelAllByCurrencyRequest, CancelAllByInstrumentRequest,CancelOrderType};
use std::fmt::{Formatter, Error as FmtError, Display};
use std::result::Result as StdResult;

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    BTC,
    ETH,
}

impl Default for Currency {
    fn default() -> Currency {
        Currency::BTC
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum AssetKind {
    Future,
    Option,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Buy,
    Sell,
    Zero, // Admin says it is a leftover from bug hunting.
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> StdResult<(), FmtError> {
        write!(f, "{:?}", self)
    }
}

impl Direction {
    pub fn sign(self) -> i64 {
        match self {
            Direction::Buy => {1}
            Direction::Sell => {-1}
            Direction::Zero => 0
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum Role {
    #[serde(rename = "M")]
    Maker,
    #[serde(rename = "T")]
    Taker,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    Limit,
    Market,
    StopLimit,
    StopMarket,
    Liquidation,
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Limit
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum OrderState {
    Open,
    Closed,
    Filled,
    Rejected,
    Cancelled,
    Untriggered,
    Archive,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TimeInForce {
    GoodTilCancelled,
    FillOrKill,
    ImmediateOrCancel,
}

impl Default for TimeInForce {
    fn default() -> Self {
        TimeInForce::GoodTilCancelled
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    IndexPrice,
    MarkPrice,
    LastPrice,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum AdvanceOption {
    #[serde(rename = "usd")]
    USD,
    #[serde(rename = "implv")]
    ImplV,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}
