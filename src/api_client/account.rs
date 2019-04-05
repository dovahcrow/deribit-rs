use crate::api_client::{DeribitAPICallResult, DeribitAPIClient};
use crate::errors::Result;
use crate::models::{GetPositionsRequest, GetPositionsResponse};

impl DeribitAPIClient {
    pub async fn private_get_positions(
        &mut self,
        req: GetPositionsRequest,
    ) -> Result<DeribitAPICallResult<Vec<GetPositionsResponse>>> {
        let resp: DeribitAPICallResult<Vec<GetPositionsResponse>> =
            await!(self.request("private/get_positions", Some(req)))?;
        Ok(resp)
    }
}
