use super::subscription::BookInstrumentNameIntervalResponse;
use crate::errors::DeribitError;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum WSMessage {
    Invoke(JSONRPCResponse),
    Subscription(JSONRPCSubscriptionResponse),
    Error(JSONRPCError),
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
pub struct JSONRPCResponse {
    pub jsonrpc: String,
    pub id: i64,
    pub testnet: bool,
    pub result: Value,
    pub us_in: i64,
    pub us_out: i64,
    pub us_diff: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct JSONRPCSubscriptionResponse {
    pub jsonrpc: String,
    pub method: String,
    pub params: SubscriptionParams,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SubscriptionParams {
    pub channel: String,
    pub data: SubscriptionData,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JSONRPCError {
    pub jsonrpc: String,
    pub id: i64,
    pub testnet: bool,
    pub error: ErrorDetail,
    pub us_in: i64,
    pub us_out: i64,
    pub us_diff: i64,
}

impl JSONRPCError {
    pub fn localize(self) -> DeribitError {
        DeribitError::RemoteError {
            code: self.error.code,
            message: self.error.message,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct ErrorDetail {
    pub code: i64,
    pub message: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SubscriptionData {
    BookInstrumentNameInterval(BookInstrumentNameIntervalResponse),
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
