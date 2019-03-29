use crate::api_client::DeribitAPIClient;
use crate::errors::Result;
use crate::models::{GetTimeResponse, HelloRequest, HelloResponse, TestRequest, TestResponse};

impl DeribitAPIClient {
    pub async fn get_time(&mut self) -> Result<GetTimeResponse> {
        Ok(await!(self.request::<_, ()>("public/get_time", None))?)
    }
    pub async fn hello<'a>(&'a mut self, req: &'a HelloRequest) -> Result<HelloResponse> {
        Ok(await!(self.request("public/hello", Some(req)))?)
    }
    pub async fn test<'a>(&'a mut self, req: &'a TestRequest) -> Result<TestResponse> {
        Ok(await!(self.request("public/test", Some(req)))?)
    }
}
