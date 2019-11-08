use crate::models::{
    Currency, Direction, Priority, Request, TransferState, TransferType, WithdrawState,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetTransfersRequest {
    currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u64>,
}

impl GetTransfersRequest {
    pub fn with_currency(currency: Currency) -> Self {
        Self {
            currency,
            count: None,
            offset: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetTransfersResponse {
    count: u64,
    data: Vec<TransferItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TransferItem {
    amount: f64,
    created_timestamp: u64,
    currency: Currency,
    direction: Direction,
    id: i64,
    other_side: String,
    state: TransferState,
    r#type: TransferType,
    updated_timestamp: i64,
}

impl Request for GetTransfersRequest {
    const METHOD: &'static str = "private/get_transfers";
    type Response = GetTransfersResponse;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SubmitTransferToSubaccountRequest {
    currency: Currency,
    amount: f64,
    destination: u64,
}

pub type SubmitTransferToSubaccountResponse = TransferItem;

impl Request for SubmitTransferToSubaccountRequest {
    const METHOD: &'static str = "private/submit_transfer_to_subaccount";
    type Response = SubmitTransferToSubaccountResponse;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SubmitTransferToUserRequest {
    currency: Currency,
    amount: f64,
    destination: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    tfa: Option<String>,
}

pub type SubmitTransferToUserResponse = TransferItem;

impl Request for SubmitTransferToUserRequest {
    const METHOD: &'static str = "private/submit_transfer_to_user";
    type Response = SubmitTransferToUserResponse;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WithdrawRequest {
    currency: Currency,
    address: String,
    amount: f64,
    priority: Priority,
    destination: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    tfa: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WithdrawResponse {
    address: String,
    amount: f64,
    confirmed_timestamp: u64,
    created_timestamp: u64,
    currency: Currency,
    fee: f64,
    id: u64,
    priority: u64,
    state: WithdrawState,
    transaction_id: String,
    updated_timestamp: u64,
}

impl Request for WithdrawRequest {
    const METHOD: &'static str = "/private/withdraw";
    type Response = WithdrawResponse;
}
