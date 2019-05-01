use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct DeribitPriceRankingData {
    pub enabled: bool,
    pub identifier: String,
    pub price: f64,
    pub timestamp: u64,
    pub weight: f64,
}