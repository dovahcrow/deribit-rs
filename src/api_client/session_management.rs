use crate::api_client::{DeribitAPICallResult, DeribitAPIClient};
use crate::errors::Result;
use crate::models::{SetHeartbeatRequest, SetHeartbeatResponse};

impl DeribitAPIClient {
    pub async fn public_set_heartbeat(
        &mut self,
        req: SetHeartbeatRequest,
    ) -> Result<DeribitAPICallResult<SetHeartbeatResponse>> {
        let resp: DeribitAPICallResult<SetHeartbeatResponse> =
            await!(self.request("public/set_heartbeat", Some(req)))?;
        Ok(resp)
    }
}
