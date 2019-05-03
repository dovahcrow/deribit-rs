use crate::models::{Request, VoidRequest};
use serde_derive::{Deserialize, Serialize};
use shrinkwraprs::Shrinkwrap;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetTimeRequest;

#[derive(Deserialize, Serialize, Debug, Clone, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct GetTimeResponse(pub i64);

impl Request for GetTimeRequest {
    const METHOD: &'static str = "public/get_time";
    type Response = GetTimeResponse;
}

impl VoidRequest for GetTimeRequest {
    #[inline]
    fn empty(&self) -> bool {
        true
    }
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TestResponse {
    pub version: String,
}

impl Request for TestRequest {
    const METHOD: &'static str = "public/test";
    type Response = TestResponse;
}
