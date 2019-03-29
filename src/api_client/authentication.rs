use crate::api_client::DeribitAPIClient;
use crate::errors::Result;
use crate::models::{AuthRequest, AuthResponse};

impl DeribitAPIClient {
    pub async fn public_auth<'a>(&'a mut self, req: &'a AuthRequest) -> Result<AuthResponse> {
        Ok(await!(self.request("public/auth", Some(req)))?)
    }
}
