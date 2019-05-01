use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeribitPriceRankingData {
    pub enabled: bool,
    pub identifier: String,
    pub price: f64,
    pub timestamp: u64,
    pub weight: f64,
}