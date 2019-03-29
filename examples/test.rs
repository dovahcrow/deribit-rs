#![feature(async_await, futures_api, await_macro)]

use deribit::errors::Result;
use deribit::models::{TestRequest};
use deribit::Deribit;
use failure::Error;
use futures::compat::Compat;
use futures::FutureExt;
use tokio::runtime::Runtime;
use env_logger::init;

fn main() -> Result<()> {
    init();
    let drb = Deribit::new();
    let mut rt = Runtime::new()?;

    let fut = async {
        let (mut client, _) = await!(drb.connect())?;
        let req = TestRequest {expected_result:Some("exception".into()) };
        let _ = await!(client.test(&req))?;

        Ok::<_, Error>(())
    };

    let fut = Compat::new(fut.boxed());
    let r = rt.block_on(fut);
    println!("aaa{:?}", r);
    Ok(())
}
