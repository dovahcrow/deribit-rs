use crate::api_client::DeribitAPIClient;
use crate::errors::Result;
use crate::models::{GetPositionsRequest, GetPositionsResponse};

impl DeribitAPIClient {
    pub async fn private_get_positions(&mut self, req: GetPositionsRequest) -> Result<Vec<GetPositionsResponse>> {
        Ok(await!(self.request("private/get_positions", Some(req)))?)
    }
}
