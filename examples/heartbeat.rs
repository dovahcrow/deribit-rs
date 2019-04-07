#![feature(async_await, futures_api, await_macro)]

use deribit::errors::Result;
use deribit::models::{Either, HeartbeatType, SetHeartbeatRequest, TestRequest};
use deribit::DeribitBuilder;
use dotenv::dotenv;
use env_logger::init;
use failure::Error;
use futures::{FutureExt, StreamExt, TryFutureExt};
use tokio::runtime::Runtime;

fn main() -> Result<()> {
    dotenv().unwrap();
    init();

    let drb = DeribitBuilder::default().testnet(true).build().unwrap();

    let mut rt = Runtime::new()?;

    let fut = async move {
        let (mut client, mut subscription) = await!(drb.connect())?;

        let resp = await!(client.call(SetHeartbeatRequest::with_interval(10)))?;
        println!("Hearbet response {:?}", await!(resp)?);

        while let Some(sub) = await!(subscription.next()) {
            match sub {
                Either::Right(l) => match l.params.r#type {
                    HeartbeatType::TestRequest => {
                        println!("Test Requested");
                        await!(client.call(TestRequest::default()))?;
                    }
                    _ => println!("Heartbeat"),
                },
                _ => {}
            }
        }

        Ok::<_, Error>(())
    };

    let fut = fut.boxed().compat();
    let r = rt.block_on(fut);
    println!("{:?}", r);
    Ok(())
}
