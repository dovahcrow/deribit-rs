use crate::api_client::DeribitAPIClient;
use crate::errors::Result;
use crate::models::{AuthRequest, AuthResponse};

impl DeribitAPIClient {
    pub async fn public_auth(&mut self, req: AuthRequest) -> Result<AuthResponse> {
        Ok(await!(self.request("public/auth", Some(req)))?)
    }
}
