pub mod account;
pub mod authentication;
pub mod market_data;
pub mod message;
pub mod session_management;
pub mod subscription;
pub mod support;
pub mod trading;

use crate::errors::DeribitError;
use failure::{Error, Fallible};
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Error as FmtError, Formatter};
use std::result::Result as StdResult;

pub use account::{
    GetAccountSummaryRequest, GetAccountSummaryResponse, GetPositionsRequest, GetPositionsResponse,
    GetSubaccountsRequest, GetSubaccountsResponse,
};
pub use authentication::{AuthRequest, AuthResponse, GrantType};
pub use market_data::{
    GetIndexRequest, GetIndexResponse, GetInstrumentsRequest, GetInstrumentsResponse,
};
pub use message::{
    HeartbeatMessage, HeartbeatMethod, JSONRPCRequest, JSONRPCResponse, JSONRPCVersion,
    SubscriptionData, SubscriptionMessage, WSMessage,
};
pub use session_management::{
    HeartbeatParams, HeartbeatType, SetHeartbeatRequest, SetHeartbeatResponse,
};
pub use subscription::{
    PrivateSubscribeRequest, PrivateUnsubscribeRequest, PublicSubscribeRequest,
    PublicUnsubscribeRequest, SubscribeResponse,
};
pub use support::{
    GetTimeRequest, GetTimeResponse, HelloRequest, HelloResponse, TestRequest, TestResponse,
};
pub use trading::{
    BuyRequest, BuyResponse, CancelAllByCurrencyRequest, CancelAllByInstrumentRequest,
    CancelAllRequest, CancelAllResponse, CancelOrderType, CancelRequest, CancelResponse,
    EditRequest, EditResponse, GetOpenOrderType, GetOpenOrdersByCurrencyRequest,
    GetOpenOrdersByCurrencyResponse, GetOpenOrdersByInstrumentRequest,
    GetOpenOrdersByInstrumentResponse, GetOrderStateRequest, GetOrderStateResponse, Order,
    SellRequest, SellResponse, Trade, TradeRequest, TradeResponse,
};

pub trait Request {
    const METHOD: &'static str;
    type Response;
}

trait VoidRequest {
    fn empty(&self) -> bool;
}

impl<R: Request> VoidRequest for R {
    #[inline]
    default fn empty(&self) -> bool {
        false
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Currency {
    #[serde(alias = "btc")]
    BTC,
    #[serde(alias = "eth")]
    ETH,
    #[serde(alias = "usd")]
    USD,
}

impl Default for Currency {
    fn default() -> Currency {
        Currency::BTC
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl std::str::FromStr for Currency {
    type Err = Error;
    fn from_str(s: &str) -> Fallible<Currency> {
        match s {
            "BTC" => Ok(Currency::BTC),
            "ETH" => Ok(Currency::ETH),
            s => Err(DeribitError::UnknownCurrency(s.to_string()).into()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AssetKind {
    Future,
    Option,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
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

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
pub enum Liquidity {
    #[serde(rename = "M")]
    Maker,
    #[serde(rename = "T")]
    Taker,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
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

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
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

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
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

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    IndexPrice,
    MarkPrice,
    LastPrice,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
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

impl<T> Either<T, T> {
    pub fn unwrap(self) -> T {
        match self {
            Either::Left(l) => l,
            Either::Right(r) => r,
        }
    }
}

impl<L, R> Either<L, R> {
    pub fn unwrap_left(self) -> L {
        match self {
            Either::Left(l) => l,
            Either::Right(_) => panic!("Either is right"),
        }
    }

    pub fn left(self) -> Option<L> {
        match self {
            Either::Left(l) => Some(l),
            Either::Right(_) => None,
        }
    }

    pub fn unwrap_right(self) -> R {
        match self {
            Either::Left(_) => panic!("Either is left"),
            Either::Right(r) => r,
        }
    }

    pub fn right(self) -> Option<R> {
        match self {
            Either::Left(_) => None,
            Either::Right(r) => Some(r),
        }
    }
}
