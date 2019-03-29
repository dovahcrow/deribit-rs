use super::subscription::channel::BookInstrumentNameIntervalMessage;
use super::subscription::channel::UserPortfolioCurrencyMessage;
use super::subscription::channel::UserTradesInstrumentNameIntervalMessage;
use crate::errors::{DeribitError, Result};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum WSMessage {
    RPC(JSONRPCResponse),
    Subscription(SubscriptionMessage),
}

#[derive(Serialize, Clone, Debug)]
pub struct JSONRPCRequest<Q> {
    pub id: i64,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Q>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JSONRPCResponse<R = Value> {
    pub jsonrpc: String,
    pub id: i64,
    pub testnet: bool,
    pub error: Option<ErrorDetail>,
    pub result: Option<R>,
    pub us_in: u128,
    pub us_out: u128,
    pub us_diff: u128,
}

impl JSONRPCResponse {
    pub fn to_result(self) -> Result<JSONRPCResponse> {
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

#[derive(Deserialize, Clone, Debug)]
pub struct SubscriptionMessage<D = SubscriptionData> {
    pub jsonrpc: String,
    pub method: String,
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
    BookInstrumentNameInterval(BookInstrumentNameIntervalMessage),
    UserPortfolioCurrency(UserPortfolioCurrencyMessage),
    UserTradesInstrumentNameInterval(Vec<UserTradesInstrumentNameIntervalMessage>),
}

#[derive(Deserialize, Clone, Debug)]
pub struct ErrorDetail {
    pub code: i64,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Either<L, R> {
    Left(L),
    Right(R),
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
