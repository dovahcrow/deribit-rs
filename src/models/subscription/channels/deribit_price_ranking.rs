use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DeribitPriceRankingData {
    pub enabled: bool,
    pub identifier: String,
    pub price: f64,
    pub timestamp: u64,
    pub weight: f64,
}
