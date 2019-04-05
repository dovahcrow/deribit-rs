use crate::api_client::{DeribitAPICallResult, DeribitAPIClient};
use crate::errors::Result;
use crate::models::{SubscribeRequest, SubscribeResponse};

impl DeribitAPIClient {
    pub async fn public_subscribe(
        &mut self,
        req: SubscribeRequest,
    ) -> Result<DeribitAPICallResult<SubscribeResponse>> {
        let resp: DeribitAPICallResult<SubscribeResponse> =
            await!(self.request("public/subscribe", Some(req)))?;
        Ok(resp)
    }
    pub async fn private_subscribe(
        &mut self,
        req: SubscribeRequest,
    ) -> Result<DeribitAPICallResult<SubscribeResponse>> {
        let resp: DeribitAPICallResult<SubscribeResponse> =
            await!(self.request("private/subscribe", Some(req)))?;
        Ok(resp)
    }
}
