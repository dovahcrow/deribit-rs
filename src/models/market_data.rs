use crate::models::{Currency, Request};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct GetIndexRequest {
    pub currency: Currency,
}

impl GetIndexRequest {
    pub fn new(currency: Currency) -> Self {
        Self { currency }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct GetIndexResponse {
    pub edp: f64,
    #[serde(flatten)]
    pub indices: HashMap<Currency, f64>,
}


impl Request for GetIndexRequest {
    const METHOD: &'static str = "public/get_index";
    type Response = GetIndexResponse;
}
