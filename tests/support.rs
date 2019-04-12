#![feature(async_await, futures_api, await_macro)]

use deribit::errors::Result;
use deribit::models::{GetTimeRequest, HelloRequest, TestRequest};
use deribit::Deribit;
use failure::Error;
use futures::{FutureExt, TryFutureExt};
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

        let _ = await!(await!(client.call(req))?)?;

        Ok::<_, Error>(())
    };

    let fut = fut.boxed().compat();
    rt.block_on(fut)?;
    Ok(())
}

#[test]
fn get_time() -> Result<()> {
    let drb = Deribit::new();
    let mut rt = Runtime::new()?;

    let fut = async {
        let (mut client, _) = await!(drb.connect())?;

        let _ = await!(await!(client.call(GetTimeRequest))?);

        Ok::<_, Error>(())
    };

    let fut = fut.boxed().compat();
    rt.block_on(fut)?;
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let drb = Deribit::new();
    let mut rt = Runtime::new()?;

    let fut = async {
        let (mut client, _) = await!(drb.connect())?;
        let req = TestRequest {
            expected_result: Some("exception".into()),
        };
        Ok::<_, Error>(await!(await!(client.call(req))?)?)
    };

    let fut = fut.boxed().compat();
    assert!(rt.block_on(fut).is_err());
    Ok(())
}
