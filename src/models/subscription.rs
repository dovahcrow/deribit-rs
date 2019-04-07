pub mod channel;

use crate::models::Request;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct PublicSubscribeRequest {
    pub channels: Vec<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct PrivateSubscribeRequest {
    pub channels: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SubscribeResponse(pub Vec<String>);

impl Request for PublicSubscribeRequest {
    const METHOD: &'static str = "public/subscribe";
    type Response = SubscribeResponse;
}

impl Request for PrivateSubscribeRequest {
    const METHOD: &'static str = "private/subscribe";
    type Response = SubscribeResponse;
}
