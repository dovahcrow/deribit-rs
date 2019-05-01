use crate::models::Request;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum HeartbeatType {
    Heartbeat,
    TestRequest,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HeartbeatParams {
    pub r#type: HeartbeatType,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SetHeartbeatRequest {
    pub interval: u64,
}

impl SetHeartbeatRequest {
    pub fn with_interval(interval: u64) -> SetHeartbeatRequest {
        SetHeartbeatRequest { interval }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum SetHeartbeatResponse {
    Ok,
}

impl Request for SetHeartbeatRequest {
    const METHOD: &'static str = "public/set_heartbeat";
    type Response = SetHeartbeatResponse;
}
