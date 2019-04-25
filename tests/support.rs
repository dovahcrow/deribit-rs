#![feature(async_await, await_macro)]

use deribit::models::{GetTimeRequest, HelloRequest, TestRequest};
use deribit::Deribit;
use failure::{Error, Fallible};
use futures::{FutureExt, TryFutureExt};
use tokio::runtime::Runtime;

#[test]
fn hello() -> Fallible<()> {
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
fn get_time() -> Fallible<()> {
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
fn test() -> Fallible<()> {
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
