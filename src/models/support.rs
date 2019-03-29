use serde_derive::{Deserialize, Serialize};

pub type GetTimeResponse = i64;

#[derive(Serialize, Debug, Clone)]
pub struct HelloRequest {
    pub client_name: String,
    pub client_version: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HelloResponse {
    pub version: String,
}


#[derive(Serialize, Debug, Clone, Default)]
pub struct TestRequest {
    pub expected_result: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TestResponse {
    pub version: String,
}
