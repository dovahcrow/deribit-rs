use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct HelloRequest {
    pub client_name: String,
    pub client_version: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HelloResponse {
    pub version: String,
}
