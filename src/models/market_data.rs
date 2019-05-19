use crate::models::{AssetKind, Currency, Request};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetBookSummaryByCurrencyRequest {
    pub currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<AssetKind>,
}

impl GetBookSummaryByCurrencyRequest {
    pub fn all(currency: Currency) -> Self {
        Self {
            currency,
            kind: None,
        }
    }

    pub fn futures(currency: Currency) -> Self {
        Self {
            currency,
            kind: Some(AssetKind::Future),
        }
    }

    pub fn options(currency: Currency) -> Self {
        Self {
            currency,
            kind: Some(AssetKind::Option),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetBookSummaryByCurrencyResponse {
    pub ask_price: Option<f64>,
    pub base_currency: Currency,
    pub bid_price: Option<f64>,
    pub creation_timestamp: u64,
    pub current_funding: Option<f64>,
    pub estimated_delivery_price: f64,
    pub funding_8h: Option<f64>,
    pub high: Option<f64>,
    pub instrument_name: String,
    pub interest_rate: Option<f64>,
    pub last: Option<f64>,
    pub low: Option<f64>,
    pub mark_price: f64,
    pub mid_price: Option<f64>,
    pub open_interest: f64,
    pub quote_currency: Currency,
    pub underlying_index: Option<String>,
    pub underlying_price: Option<f64>,
    pub volume: f64,
    pub volume_usd: Option<f64>,
}

impl Request for GetBookSummaryByCurrencyRequest {
    const METHOD: &'static str = "public/get_book_summary_by_currency";
    type Response = Vec<GetBookSummaryByCurrencyResponse>;
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetIndexRequest {
    pub currency: Currency,
}

impl GetIndexRequest {
    pub fn new(currency: Currency) -> Self {
        Self { currency }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetIndexResponse {
    pub edp: f64,
    #[serde(flatten)]
    pub indices: HashMap<Currency, f64>,
}


impl Request for GetIndexRequest {
    const METHOD: &'static str = "public/get_index";
    type Response = GetIndexResponse;
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
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

    pub fn futures(currency: Currency) -> Self {
        Self::with_kind(currency, AssetKind::Future)
    }

    pub fn options(currency: Currency) -> Self {
        Self::with_kind(currency, AssetKind::Option)
    }

    pub fn with_kind(currency: Currency, kind: AssetKind) -> Self {
        Self {
            currency,
            kind: Some(kind),
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
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
