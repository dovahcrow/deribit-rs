use crate::api_client::{DeribitAPICallResult, DeribitAPIClient};
use crate::errors::Result;
use crate::models::{
    BuyRequest, BuyResponse, CancelAllByCurrencyRequest, CancelAllByInstrumentRequest,
    CancelResponse, SellRequest, SellResponse,
};

impl DeribitAPIClient {
    pub async fn private_buy(
        &mut self,
        req: BuyRequest,
    ) -> Result<DeribitAPICallResult<BuyResponse>> {
        Ok(await!(self.request("private/buy", Some(req)))?)
    }

    pub async fn private_sell(
        &mut self,
        req: SellRequest,
    ) -> Result<DeribitAPICallResult<SellResponse>> {
        let resp: DeribitAPICallResult<SellResponse> =
            await!(self.request("private/sell", Some(req)))?;
        Ok(resp)
    }

    pub async fn private_cancel_all(&mut self) -> Result<DeribitAPICallResult<CancelResponse>> {
        let resp: DeribitAPICallResult<CancelResponse> =
            await!(self.request::<_, ()>("private/cancel_all", None))?;
        Ok(resp)
    }

    pub async fn private_cancel_all_by_instrument(
        &mut self,
        req: CancelAllByInstrumentRequest,
    ) -> Result<DeribitAPICallResult<CancelResponse>> {
        let resp: DeribitAPICallResult<CancelResponse> =
            await!(self.request("private/cancel_all_by_instrument", Some(req)))?;
        Ok(resp)
    }

    pub async fn private_cancel_all_by_currency(
        &mut self,
        req: CancelAllByCurrencyRequest,
    ) -> Result<DeribitAPICallResult<CancelResponse>> {
        let resp: DeribitAPICallResult<CancelResponse> =
            await!(self.request("private/cancel_all_by_currency", Some(req)))?;
        Ok(resp)
    }
}
