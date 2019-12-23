use crate::models::Request;
use serde::{Deserialize, Serialize};
use shrinkwraprs::Shrinkwrap;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetTimeRequest;

#[derive(Deserialize, Serialize, Debug, Clone, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct GetTimeResponse(pub i64);

impl Request for GetTimeRequest {
    const METHOD: &'static str = "public/get_time";
    const HAS_PAYLOAD: bool = false;
    type Response = GetTimeResponse;
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct HelloRequest {
    pub client_name: String,
    pub client_version: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HelloResponse {
    pub version: String,
}

impl Request for HelloRequest {
    const METHOD: &'static str = "public/hello";
    type Response = HelloResponse;
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TestRequest {
    pub expected_result: Option<String>,
}

impl TestRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn expect(result: &str) -> Self {
        Self {
            expected_result: Some(result.into()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TestResponse {
    pub version: String,
}

impl Request for TestRequest {
    const METHOD: &'static str = "public/test";
    type Response = TestResponse;
}
