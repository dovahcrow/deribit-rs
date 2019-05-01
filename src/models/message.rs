use super::session_management::HeartbeatParams;
use super::subscription::{
    AnnouncementsData, BookData, DeribitPriceIndexData, DeribitPriceRankingData,
    EstimatedExpirationPriceData, GroupedBookData, MarkPriceOptionData, PerpetualData, QuoteData,
    TickerData, TradesData, UserOrdersData, UserPortfolioData, UserTradesData,
};
use crate::errors::DeribitError;
use crate::models::Request;
use failure::Fallible;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum WSMessage {
    RPC(JSONRPCResponse),
    Subscription(SubscriptionMessage),
    Heartbeat(HeartbeatMessage),
}

#[derive(Serialize, Clone, Debug)]
pub struct JSONRPCRequest<Q: Request> {
    pub id: i64,
    pub method: String,
    #[serde(skip_serializing_if = "crate::models::EmptyRequest::empty")]
    pub params: Q,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JSONRPCResponse<R = Value> {
    pub jsonrpc: JSONRPCVersion,
    pub id: i64,
    pub testnet: bool,
    pub error: Option<ErrorDetail>,
    pub result: Option<R>,
    pub us_in: u64,
    pub us_out: u64,
    pub us_diff: u64,
}

impl JSONRPCResponse {
    pub fn to_result(self) -> Fallible<JSONRPCResponse> {
        if let Some(err) = self.error {
            Err(DeribitError::RemoteError {
                code: err.code,
                message: err.message,
            }
            .into())
        } else if let Some(_) = self.result {
            Ok(self)
        } else {
            unreachable!()
        }
    }
}

#[derive(Deserialize, Clone, Debug, Copy)]
pub enum JSONRPCVersion {
    #[serde(rename = "2.0")]
    V2,
}

#[derive(Deserialize, Clone, Debug, Copy)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionMethod {
    Subscription,
}

#[derive(Deserialize, Clone, Debug, Copy)]
#[serde(rename_all = "lowercase")]
pub enum HeartbeatMethod {
    Heartbeat,
}

#[derive(Deserialize, Clone, Debug)]
pub struct HeartbeatMessage {
    pub jsonrpc: JSONRPCVersion,
    pub method: HeartbeatMethod,
    pub params: HeartbeatParams,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SubscriptionMessage<D = SubscriptionData> {
    pub jsonrpc: JSONRPCVersion,
    pub method: SubscriptionMethod,
    pub params: SubscriptionParams<D>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SubscriptionParams<D = SubscriptionData> {
    pub channel: String,
    pub data: D,
}

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Clone, Debug)]
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
