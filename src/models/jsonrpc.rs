use crate::models::{Either, Request};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct JSONRPCRequest<Q: Request> {
    pub id: i64,
    pub method: String,
    #[serde(skip_serializing_if = "crate::models::Request::no_payload")]
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
