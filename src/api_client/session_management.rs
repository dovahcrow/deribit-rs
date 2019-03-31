use crate::api_client::DeribitAPIClient;
use crate::errors::Result;
use crate::models::SetHeartbeatRequest;

impl DeribitAPIClient {
    pub async fn public_set_heartbeat(&mut self, req: SetHeartbeatRequest) -> Result<String> {
        Ok(await!(self.request("public/set_heartbeat", Some(req)))?)
    }
}
