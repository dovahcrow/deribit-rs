mod account;
mod authentication;
mod message;
mod session_management;
mod subscription;
mod support;
mod trading;

use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Error as FmtError, Formatter};
use std::result::Result as StdResult;

pub use account::{GetPositionsRequest, GetPositionsResponse};
pub use authentication::{AuthRequest, AuthResponse, GrantType};
pub use message::{
    HeartbeatMessage, JSONRPCRequest, JSONRPCResponse, SubscriptionData, SubscriptionMessage,
    WSMessage,
};
pub use session_management::{HeartbeatParams, HeartbeatType, SetHeartbeatRequest};
pub use subscription::channel::{
    BookInstrumentNameIntervalData, BookInstrumentNameIntervalRequest, Delta, OrderBookDelta,
};
pub use subscription::channel::{UserPortfolioCurrencyData, UserPortfolioCurrencyRequest};
pub use subscription::channel::{
    UserTradesInstrumentNameIntervalData, UserTradesInstrumentNameIntervalRequest,
};
pub use subscription::{SubscribeRequest, SubscribeResponse};
pub use support::{GetTimeResponse, HelloRequest, HelloResponse, TestRequest, TestResponse};
pub use trading::{
    BuyRequest, BuyResponse, CancelAllByCurrencyRequest, CancelAllByInstrumentRequest,
    CancelOrderType, CancelResponse, Order, SellRequest, SellResponse, Trade, TradeRequest,
    TradeResponse,
};

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
            Direction::Buy => 1,
            Direction::Sell => -1,
            Direction::Zero => 0,
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

impl<L, R> Either<L, R> {
    pub fn map_left<F, U>(self, f: F) -> Either<U, R>
    where
        F: FnOnce(L) -> U,
    {
        match self {
            Either::Left(l) => Either::Left(f(l)),
            Either::Right(r) => Either::Right(r),
        }
    }

    pub fn map_right<F, U>(self, f: F) -> Either<L, U>
    where
        F: FnOnce(R) -> U,
    {
        match self {
            Either::Right(r) => Either::Right(f(r)),
            Either::Left(l) => Either::Left(l),
        }
    }
}
