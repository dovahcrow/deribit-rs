use crate::models::{AssetKind, Currency, Request};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct GetIndexRequest {
    pub currency: Currency,
}

impl GetIndexRequest {
    pub fn new(currency: Currency) -> Self {
        Self { currency }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct GetIndexResponse {
    pub edp: f64,
    #[serde(flatten)]
    pub indices: HashMap<Currency, f64>,
}


impl Request for GetIndexRequest {
    const METHOD: &'static str = "public/get_index";
    type Response = GetIndexResponse;
}


#[derive(Serialize, Default)]
pub struct GetInstrumentsRequest {
    pub currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<AssetKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
}

impl GetInstrumentsRequest {
    pub fn new(currency: Currency) -> Self {
        Self {
            currency,
            ..Default::default()
        }
    }

    pub fn expired(currency: Currency) -> Self {
        Self {
            currency,
            expired: Some(true),
            ..Default::default()
        }
    }

    pub fn with_kind(currency: Currency, kind: AssetKind) -> Self {
        Self {
            currency,
            kind: Some(kind),
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct GetInstrumentsResponse {
    pub base_currency: String,
    pub contract_size: f64,
    pub creation_timestamp: u64,
    pub expiration_timestamp: u64,
    pub instrument_name: String,
    pub is_active: bool,
    pub kind: AssetKind,
    pub min_trade_amount: f64,
    pub option_type: Option<String>,
    pub quote_currency: Option<Currency>,
    pub settlement_period: String,
    pub strike: Option<f64>,
    pub tick_size: f64,
}


impl Request for GetInstrumentsRequest {
    const METHOD: &'static str = "public/get_instruments";
    type Response = Vec<GetInstrumentsResponse>;
}
