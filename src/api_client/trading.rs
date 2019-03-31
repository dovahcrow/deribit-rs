use crate::api_client::DeribitAPIClient;
use crate::errors::Result;
use crate::models::{BuyRequest, BuyResponse, SellRequest, SellResponse, CancelResponse, CancelAllByInstrumentRequest, CancelAllByCurrencyRequest};

impl DeribitAPIClient {
    pub async fn private_buy(&mut self, req: BuyRequest) -> Result<BuyResponse> {
        Ok(await!(self.request("private/buy", Some(req)))?)
    }

    pub async fn private_sell(&mut self, req: SellRequest) -> Result<SellResponse> {
        Ok(await!(self.request("private/sell", Some(req)))?)
    }

    pub async fn private_cancel_all(&mut self) -> Result<CancelResponse> {
        Ok(await!(self.request::<_, ()>("private/cancel_all", None))?)
    }

    pub async fn private_cancel_all_by_instrument(&mut self, req: CancelAllByInstrumentRequest) -> Result<CancelResponse> {
        Ok(await!(self.request("private/cancel_all_by_instrument", Some(req)))?)
    }

    pub async fn private_cancel_all_by_currency(&mut self, req: CancelAllByCurrencyRequest) -> Result<CancelResponse> {
        Ok(await!(self.request("private/cancel_all_by_currency", Some(req)))?)
    }
}
