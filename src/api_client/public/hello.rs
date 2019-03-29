use crate::api_client::DeribitAPIClient;
use crate::errors::Result;
use crate::models::{HelloRequest, HelloResponse};

impl DeribitAPIClient {
    pub async fn hello<'a>(&'a mut self, req: &'a HelloRequest) -> Result<HelloResponse> {
        Ok(await!(self.request("public/hello", Some(req)))?)
    }
}
