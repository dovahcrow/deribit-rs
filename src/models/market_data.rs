use crate::models::{AssetKind, Currency, Request};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::subscription::{Greeks, Stats};

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
    pub estimated_delivery_price: Option<f64>,
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

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct GetFundingRateValueRequest {
    pub instrument_name: String,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
}

impl GetFundingRateValueRequest {
    pub fn new(instrument_name: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self {
            instrument_name: instrument_name.to_string(),
            start_timestamp: start.timestamp_millis() as u64,
            end_timestamp: end.timestamp_millis() as u64,
        }
    }
}

pub type GetFundingRateValueResponse = f64;

impl Request for GetFundingRateValueRequest {
    const METHOD: &'static str = "public/get_funding_rate_value";
    type Response = GetFundingRateValueResponse;
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct GetOrderBookRequest {
    instrument_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    depth: Option<u64>,
}

impl GetOrderBookRequest {
    pub fn new(instrument_name: &str) -> Self {
        Self {
            instrument_name: instrument_name.to_string(),
            ..Default::default()
        }
    }
    pub fn with_depth(instrument_name: &str, depth: u64) -> Self {
        Self {
            instrument_name: instrument_name.to_string(),
            depth: Some(depth),
        }
    }
}

impl Request for GetOrderBookRequest {
    const METHOD: &'static str = "public/get_order_book";
    type Response = GetOrderBookResponse;
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetOrderBookResponse {
    pub ask_iv: Option<f64>,
    pub asks: Vec<Ask>,
    pub best_ask_amount: f64,
    pub best_ask_price: Option<f64>,
    pub best_bid_amount: f64,
    pub best_bid_price: Option<f64>,
    pub bid_iv: Option<f64>,
    pub bids: Vec<Bid>,
    pub current_funding: Option<f64>,
    pub delivery_price: Option<f64>,
    pub funding_8h: Option<f64>,
    pub greeks: Option<Greeks>,
    pub index_price: f64,
    pub instrument_name: String,
    pub interest_rate: Option<f64>,
    pub last_price: Option<f64>,
    pub mark_iv: Option<f64>,
    pub mark_price: f64,
    pub max_price: f64,
    pub min_price: f64,
    pub open_interest: f64,
    pub settlement_price: Option<f64>,
    pub state: State,
    pub stats: Stats,
    pub timestamp: u64,
    pub underlying_index: Option<String>,
    pub underlying_price: Option<f64>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Ask(f64, f64);

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Bid(f64, f64);

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum State {
    #[serde(alias = "open")]
    Open,
    #[serde(alias = "closed")]
    Closed,
}
