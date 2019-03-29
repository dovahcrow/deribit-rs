use crate::api_client::DeribitAPIClient;
use crate::errors::Result;
use crate::models::{SubscribeRequest, SubscribeResponse};

impl DeribitAPIClient {
    pub async fn subscribe<'a>(&'a mut self, req: &'a SubscribeRequest) -> Result<SubscribeResponse> {
        Ok(await!(self.request("public/subscribe", Some(req)))?)
    }
}
