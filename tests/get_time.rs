#![feature(async_await, futures_api, await_macro)]

use deribit::errors::Result;
use deribit::Deribit;
use failure::Error;
use futures::compat::Compat;
use futures::FutureExt;
use tokio::runtime::Runtime;

#[test]
fn get_time() -> Result<()> {
    let drb = Deribit::new();
    let mut rt = Runtime::new()?;

    let fut = async {
        let (mut client, _) = await!(drb.connect())?;

        let _ = await!(client.get_time())?;

        Ok::<_, Error>(())
    };

    let fut = Compat::new(fut.boxed());
    rt.block_on(fut)?;
    Ok(())
}
