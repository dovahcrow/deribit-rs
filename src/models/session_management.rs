use crate::models::Request;
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DisableHeartbeatRequest;

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum DisableHeartbeatResponse {
    Ok,
}

impl Request for DisableHeartbeatRequest {
    const METHOD: &'static str = "public/disable_heartbeat";
    const HAS_PAYLOAD: bool = false;
    type Response = DisableHeartbeatResponse;
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum CancelOnDisconnectScope {
    Connection,
    Account,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct EnableCancelOnDisconnectRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<CancelOnDisconnectScope>,
}

impl EnableCancelOnDisconnectRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_scope(scope: CancelOnDisconnectScope) -> Self {
        Self { scope: Some(scope) }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum EnableCancelOnDisconnectResponse {
    Ok,
}

impl Request for EnableCancelOnDisconnectRequest {
    const METHOD: &'static str = "private/enable_cancel_on_disconnect";
    type Response = EnableCancelOnDisconnectResponse;
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct DisableCancelOnDisconnectRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<CancelOnDisconnectScope>,
}

impl DisableCancelOnDisconnectRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_scope(scope: CancelOnDisconnectScope) -> Self {
        Self { scope: Some(scope) }
    }
}
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum DisableCancelOnDisconnectResponse {
    Ok,
}

impl Request for DisableCancelOnDisconnectRequest {
    const METHOD: &'static str = "private/disable_cancel_on_disconnect";
    type Response = DisableCancelOnDisconnectRequest;
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct GetCancelOnDisconnectRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<CancelOnDisconnectScope>,
}

impl GetCancelOnDisconnectRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_scope(scope: CancelOnDisconnectScope) -> Self {
        Self { scope: Some(scope) }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct GetCancelOnDisconnectResponse {
    enabled: bool,
    scope: CancelOnDisconnectScope,
}

impl Request for GetCancelOnDisconnectRequest {
    const METHOD: &'static str = "private/get_cancel_on_disconnect";
    type Response = GetCancelOnDisconnectResponse;
}
