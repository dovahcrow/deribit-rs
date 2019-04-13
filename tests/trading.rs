#![feature(async_await, futures_api, await_macro)]

use deribit::errors::Result;
use deribit::models::{AuthRequest, GetOrderStateRequest};
use deribit::DeribitBuilder;
use dotenv::dotenv;
use env_logger::init;
use failure::Error;
use futures::{FutureExt, TryFutureExt};
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_order_state() -> Result<()> {
    dotenv()?;
    init();

    let key = var("DERIBIT_KEY").unwrap();
    let secret = var("DERIBIT_SECRET").unwrap();

    let drb = DeribitBuilder::default().testnet(true).build().unwrap();
    let mut rt = Runtime::new()?;

    let fut = async move {
        let (mut client, _) = await!(drb.connect())?;
        let req = AuthRequest::credential_auth(&key, &secret);
        let _ = await!(await!(client.call(req))?)?;

        let req = GetOrderStateRequest::new("2260120935");
        Ok::<_, Error>(await!(await!(client.call(req))?)?)
    };

    let fut = fut.boxed().compat();
    let _ = rt.block_on(fut)?;
    Ok(())
}
