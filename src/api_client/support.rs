use crate::api_client::{DeribitAPICallResult, DeribitAPIClient};
use crate::errors::Result;
use crate::models::{GetTimeResponse, HelloRequest, HelloResponse, TestRequest, TestResponse};

impl DeribitAPIClient {
    pub async fn public_get_time(&mut self) -> Result<DeribitAPICallResult<GetTimeResponse>> {
        let resp: DeribitAPICallResult<GetTimeResponse> =
            await!(self.request::<_, ()>("public/get_time", None))?;
        Ok(resp)
    }
    pub async fn public_hello(
        &mut self,
        req: HelloRequest,
    ) -> Result<DeribitAPICallResult<HelloResponse>> {
        let resp: DeribitAPICallResult<HelloResponse> =
            await!(self.request("public/hello", Some(req)))?;
        Ok(resp)
    }
    pub async fn public_test(
        &mut self,
        req: TestRequest,
    ) -> Result<DeribitAPICallResult<TestResponse>> {
        let resp: DeribitAPICallResult<TestResponse> =
            await!(self.request("public/test", Some(req)))?;
        Ok(resp)
    }
}
