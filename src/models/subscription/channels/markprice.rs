use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MarkPriceOptionData {
    pub synthetic_future: Option<SyntheticFuture>,
    pub instrument_name: String,
    pub iv: f64,
    pub mark_price: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SyntheticFuture {
    pub mark_price: f64,
    pub instrument_name: String,
}