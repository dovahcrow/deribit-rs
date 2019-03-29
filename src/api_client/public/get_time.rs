use crate::api_client::DeribitAPIClient;
use crate::errors::Result;
use crate::models::{GetTimeResponse};

impl DeribitAPIClient {
    pub async fn get_time(&mut self) -> Result<GetTimeResponse> {
        Ok(await!(self.request::<_, ()>("public/get_time", None))?)
    }
}
