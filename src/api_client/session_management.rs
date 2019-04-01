use crate::api_client::DeribitAPIClient;
use crate::errors::Result;
use crate::models::{SetHeartbeatRequest, SetHeartbeatResponse};

impl DeribitAPIClient {
    pub async fn public_set_heartbeat(
        &mut self,
        req: SetHeartbeatRequest,
    ) -> Result<SetHeartbeatResponse> {
        Ok(await!(self.request("public/set_heartbeat", Some(req)))?)
    }
}
