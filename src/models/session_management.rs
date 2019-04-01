use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum HeartbeatType {
    Heartbeat,
    TestRequest,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HeartbeatParams {
    pub r#type: HeartbeatType,
}

#[derive(Serialize, Debug, Clone)]
pub struct SetHeartbeatRequest {
    pub interval: u64,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum SetHeartbeatResponse {
    Ok,
}
