use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct EstimatedExpirationPriceData {
    pub is_estimated: bool,
    pub price: f64,
    pub seconds: f64,
}
