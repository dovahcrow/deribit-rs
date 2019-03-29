use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct SubscribeRequest {
    pub channels: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SubscribeResponse(Vec<String>);
