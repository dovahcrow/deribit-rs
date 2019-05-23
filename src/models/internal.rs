use super::subscription::{
    AnnouncementsData, BookData, DeribitPriceIndexData, DeribitPriceRankingData,
    EstimatedExpirationPriceData, GroupedBookData, MarkPriceOptionData, PerpetualData, QuoteData,
    TickerData, TradesData, UserOrdersData, UserPortfolioData, UserTradesData,
};
use crate::models::{Either, Request};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct JSONRPCRequest<Q: Request> {
    pub id: i64,
    pub method: String,
    #[serde(skip_serializing_if = "crate::models::VoidRequest::empty")]
    pub params: Q,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JSONRPCResponse<R> {
    pub jsonrpc: JSONRPCVersion,
    pub id: i64,
    pub testnet: bool,
    #[serde(alias = "error")]
    pub result: Either<R, ErrorDetail>,
    pub us_in: u64,
    pub us_out: u64,
    pub us_diff: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug, Copy)]
pub enum JSONRPCVersion {
    #[serde(rename = "2.0")]
    V2,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SubscriptionMessage {
    pub jsonrpc: JSONRPCVersion,
    pub method: SubscriptionMethod,
    pub params: SubscriptionParams,
}

impl SubscriptionMessage {
    pub fn is_subscription(&self) -> bool {
        self.method.is_subscription()
    }
    pub fn is_heartbeat(&self) -> bool {
        self.method.is_heartbeat()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Copy)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionMethod {
    Subscription,
    Heartbeat,
}

impl SubscriptionMethod {
    pub fn is_subscription(self) -> bool {
        match self {
            SubscriptionMethod::Subscription => true,
            SubscriptionMethod::Heartbeat => false,
        }
    }
    pub fn is_heartbeat(&self) -> bool {
        match self {
            SubscriptionMethod::Subscription => false,
            SubscriptionMethod::Heartbeat => true,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SubscriptionParams {
    Subscription {
        channel: String,
        data: SubscriptionData,
    },
    Heartbeat {
        r#type: HeartbeatType,
    },
}


impl SubscriptionParams {
    pub fn is_subscription(&self) -> bool {
        match self {
            SubscriptionParams::Subscription { .. } => true,
            SubscriptionParams::Heartbeat { .. } => false,
        }
    }
    pub fn is_heartbeat(&self) -> bool {
        match self {
            SubscriptionParams::Subscription { .. } => false,
            SubscriptionParams::Heartbeat { .. } => true,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum HeartbeatType {
    Heartbeat,
    TestRequest,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum SubscriptionData {
    Announcements(AnnouncementsData),
    Book(BookData),
    DeribitPriceIndex(DeribitPriceIndexData),
    DeribitPriceRanking(Vec<DeribitPriceRankingData>),
    EstimatedExpirationPrice(EstimatedExpirationPriceData),
    GroupedBook(GroupedBookData),
    MarkPriceOption(Vec<MarkPriceOptionData>),
    Perpetual(PerpetualData),
    Quote(QuoteData),
    Ticker(TickerData),
    Trades(Vec<TradesData>),
    UserOrders(UserOrdersData),
    UserOrdersBatch(Vec<UserOrdersData>),
    UserPortfolio(UserPortfolioData),
    UserTrades(Vec<UserTradesData>),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ErrorDetail {
    pub code: i64,
    pub message: String,
}

// {
//     "jsonrpc": "2.0",
//     "id": 5239,
//     "testnet": false,
//     "result": [
//         {
//             "currency": "BTC",
//             "currencyLong": "Bitcoin",
//             "minConfirmation": 2,
//             "txFee": 0.0006,
//             "isActive": true,
//             "coinType": "BITCOIN",
//             "baseAddress": null
//         }
//     ],
//     "usIn": 1535043730126248,
//     "usOut": 1535043730126250,
//     "usDiff": 2
// }
