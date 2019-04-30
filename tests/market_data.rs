#![feature(async_await, await_macro)]

use deribit::models::{Currency, GetIndexRequest, GetInstrumentsRequest};
use deribit::DeribitBuilder;
use dotenv::dotenv;
use failure::{Error, Fallible};
use futures::{FutureExt, TryFutureExt};
use tokio::runtime::Runtime;

#[test]
fn get_index() -> Fallible<()> {
    let _ = dotenv();

    let drb = DeribitBuilder::default().testnet(true).build().unwrap();
    let mut rt = Runtime::new()?;

    let fut = async move {
        let (mut client, _) = await!(drb.connect())?;
        let req = GetIndexRequest::new(Currency::BTC);
        let _ = await!(await!(client.call(req))?)?;
        let req = GetIndexRequest::new(Currency::ETH);
        let _ = await!(await!(client.call(req))?)?;

        Ok::<_, Error>(())
    };

    let fut = fut.boxed().compat();
    let _ = rt.block_on(fut)?;
    Ok(())
}


#[test]
fn get_instruments() -> Fallible<()> {
    let _ = dotenv();

    let drb = DeribitBuilder::default().build().unwrap();
    let mut rt = Runtime::new()?;

    let fut = async move {
        let (mut client, _) = await!(drb.connect())?;
        let req = GetInstrumentsRequest::new(Currency::BTC);
        let _ = await!(await!(client.call(req))?)?;
        let req = GetInstrumentsRequest::expired(Currency::ETH);
        let _ = await!(await!(client.call(req))?)?;

        Ok::<_, Error>(())
    };

    let fut = fut.boxed().compat();
    let _ = rt.block_on(fut)?;
    Ok(())
}
