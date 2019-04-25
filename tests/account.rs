#![feature(async_await, await_macro)]

use deribit::models::{AuthRequest, Currency, GetAccountSummaryRequest};
use deribit::DeribitBuilder;
use dotenv::dotenv;
use failure::{Error, Fallible};
use futures::{FutureExt, TryFutureExt};
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_account_summary() -> Fallible<()> {
    dotenv()?;


    let key = var("DERIBIT_KEY").unwrap();
    let secret = var("DERIBIT_SECRET").unwrap();

    let drb = DeribitBuilder::default().testnet(true).build().unwrap();
    let mut rt = Runtime::new()?;

    let fut = async move {
        let (mut client, _) = await!(drb.connect())?;
        let req = AuthRequest::credential_auth(&key, &secret);
        let _ = await!(await!(client.call(req))?)?;

        let req = GetAccountSummaryRequest::extended(Currency::BTC);
        Ok::<_, Error>(await!(await!(client.call(req))?)?)
    };

    let fut = fut.boxed().compat();
    let _ = rt.block_on(fut)?;
    Ok(())
}
