use crate::models::{AssetKind, Currency, Request, OrderState};
use serde::{Deserialize, Serialize};
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetBookSummaryByInstrumentRequest {
    pub instrument_name: Option<String>,
}

impl GetBookSummaryByInstrumentRequest {
    pub fn instrument(instrument_name: &str) -> Self {
        Self {
            instrument_name: Some(instrument_name.into()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GetBookSummaryByInstrumentResponse {
    pub volume: Option<f64>,
    pub underlying_price: Option<f64>,
    pub underlying_index: Option<String>,
    pub quote_currency: Option<Currency>,
    pub price_change: Option<f64>,
    pub open_interest: Option<f64>,
    pub mid_price: Option<f64>,
    pub mark_price: Option<f64>,
    pub low: Option<f64>,
    pub last: Option<f64>,
    pub interest_rate: Option<f64>,
    pub instrument_name: Option<String>,
    pub high: Option<f64>,
    pub creation_timestamp: Option<u64>,
    pub bid_price: Option<String>,
    pub base_currency: Option<String>,
    pub ask_price: Option<f64>,
}

impl Request for GetBookSummaryByInstrumentRequest {
    const METHOD: &'static str = "public/get_book_summary_by_instrument";
    type Response = GetBookSummaryByInstrumentResponse;
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TickerRequest {
    pub instrument_name: Option<String>,
}

impl TickerRequest {
    pub fn instrument(instrument_name: &str) -> Self {
        Self {
            instrument_name: Some(instrument_name.into()),
        }
    }
}

impl Request for TickerRequest {
    const METHOD: &'static str = "public/ticker";
    type Response = TickerResponse;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TickerResponse {
    pub ask_iv: Option<f64>,
    pub best_ask_amount: f64,
    pub best_ask_price: Option<f64>,
    pub best_bid_amount: f64,
    pub best_bid_price: Option<f64>,
    pub bid_iv: Option<f64>,
    pub current_funding: Option<f64>,
    pub estimated_delivery_price: Option<f64>,
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
    pub state: OrderState,
    pub stats: Stats,
    pub timestamp: u64,
    pub underlying_index: Option<String>,
    pub underlying_price: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Stats {
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub volume: Option<f64>,
    pub price_change: Option<f64>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Greeks {
    pub delta: f64,
    pub gamma: f64,
    pub rho: f64,
    pub theta: f64,
    pub vega: f64,
}