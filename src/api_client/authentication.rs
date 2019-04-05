use crate::api_client::{DeribitAPICallResult, DeribitAPIClient};
use crate::errors::Result;
use crate::models::{AuthRequest, AuthResponse};

impl DeribitAPIClient {
    pub async fn public_auth(
        &mut self,
        req: AuthRequest,
    ) -> Result<DeribitAPICallResult<AuthResponse>> {
        let resp: DeribitAPICallResult<AuthResponse> =
            await!(self.request("public/auth", Some(req)))?;
        Ok(resp)
    }
}
