#![feature(async_await, futures_api, await_macro)]

use deribit::errors::Result;
use deribit::models::HelloRequest;
use deribit::Deribit;
use failure::Error;
use futures::compat::Compat;
use futures::FutureExt;
use tokio::runtime::Runtime;

#[test]
fn hello() -> Result<()> {
    let drb = Deribit::new();
    let mut rt = Runtime::new()?;

    let fut = async {
        let (mut client, _) = await!(drb.connect())?;

        let req = HelloRequest {
            client_name: "deribit-rs".into(),
            client_version: "0.0.1".into(),
        };

        let _ = await!(client.hello(&req))?;

        Ok::<_, Error>(())
    };

    let fut = Compat::new(fut.boxed());
    rt.block_on(fut)?;
    Ok(())
}
