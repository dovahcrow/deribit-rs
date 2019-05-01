use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeribitPriceIndexData {
    pub index_name: String,
    pub price: f64,
    pub timestamp: u64,
}