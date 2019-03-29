use crate::api_client::DeribitAPIClient;
use crate::errors::Result;
use crate::models::{BuyRequest, BuyResponse, SellRequest, SellResponse};

impl DeribitAPIClient {
    pub async fn private_buy<'a>(&'a mut self, req: &'a BuyRequest) -> Result<BuyResponse> {
        Ok(await!(self.request("private/buy", Some(req)))?)
    }
    pub async fn private_sell<'a>(&'a mut self, req: &'a SellRequest) -> Result<SellResponse> {
        Ok(await!(self.request("private/sell", Some(req)))?)
    }
}
