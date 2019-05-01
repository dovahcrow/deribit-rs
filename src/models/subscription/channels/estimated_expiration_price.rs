use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct EstimatedExpirationPriceData {
    pub is_estimated: bool,
    pub price: f64,
    pub seconds: f64,
}