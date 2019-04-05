#![feature(async_await, futures_api, await_macro)]

use deribit::errors::Result;
use deribit::models::{HelloRequest, TestRequest};
use deribit::Deribit;
use failure::Error;
use futures::compat::Compat;
use futures::FutureExt;
use tokio::runtime::Runtime;

#[test]
fn public_hello() -> Result<()> {
    let drb = Deribit::new();
    let mut rt = Runtime::new()?;

    let fut = async {
        let (mut client, _) = await!(drb.connect())?;

        let req = HelloRequest {
            client_name: "deribit-rs".into(),
            client_version: "0.0.1".into(),
        };

        let _ = await!(await!(client.public_hello(req))?)?;

        Ok::<_, Error>(())
    };

    let fut = Compat::new(fut.boxed());
    rt.block_on(fut)?;
    Ok(())
}

#[test]
fn public_get_time() -> Result<()> {
    let drb = Deribit::new();
    let mut rt = Runtime::new()?;

    let fut = async {
        let (mut client, _) = await!(drb.connect())?;

        let _ = await!(await!(client.public_get_time())?);

        Ok::<_, Error>(())
    };

    let fut = Compat::new(fut.boxed());
    rt.block_on(fut)?;
    Ok(())
}

#[test]
fn public_test() -> Result<()> {
    let drb = Deribit::new();
    let mut rt = Runtime::new()?;

    let fut = async {
        let (mut client, _) = await!(drb.connect())?;
        let req = TestRequest {
            expected_result: Some("exception".into()),
        };
        Ok::<_, Error>(await!(await!(client.public_test(req))?)?)
    };

    let fut = Compat::new(fut.boxed());
    let r = rt.block_on(fut);

    assert!(r.is_err());
    Ok(())
}
