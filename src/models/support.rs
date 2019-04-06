use crate::models::{EmptyRequest, Request};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct GetTimeRequest;
pub type GetTimeResponse = i64;

impl Request for GetTimeRequest {
    const METHOD: &'static str = "public/get_time";
    type Response = GetTimeResponse;
}

impl EmptyRequest for GetTimeRequest {
    #[inline]
    fn empty(&self) -> bool {
        true
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct HelloRequest {
    pub client_name: String,
    pub client_version: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HelloResponse {
    pub version: String,
}

impl Request for HelloRequest {
    const METHOD: &'static str = "public/hello";
    type Response = HelloResponse;
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct TestRequest {
    pub expected_result: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TestResponse {
    pub version: String,
}

impl Request for TestRequest {
    const METHOD: &'static str = "public/test";
    type Response = TestResponse;
}
