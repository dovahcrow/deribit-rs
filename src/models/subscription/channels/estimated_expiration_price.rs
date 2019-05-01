use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EstimatedExpirationPriceData {
    pub is_estimated: bool,
    pub price: f64,
    pub seconds: f64,
}