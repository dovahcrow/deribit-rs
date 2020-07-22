use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]

pub struct PerpetualData {
    pub interest: f64,
}
