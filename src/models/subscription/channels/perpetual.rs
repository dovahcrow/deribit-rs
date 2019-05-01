use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PerpetualData {
    pub interest: f64,
}
