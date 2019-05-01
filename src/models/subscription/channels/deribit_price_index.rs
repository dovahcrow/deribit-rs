use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct DeribitPriceIndexData {
    pub index_name: String,
    pub price: f64,
    pub timestamp: u64,
}