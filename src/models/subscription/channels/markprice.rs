use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MarkPriceOptionData {
    pub instrument_name: String,
    pub iv: f64,
    pub mark_price: f64,
}
