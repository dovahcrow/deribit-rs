#![feature(async_await, futures_api, await_macro)]

use deribit::errors::Result;
use deribit::models::{Either, HeartbeatType, SetHeartbeatRequest, TestRequest};
use deribit::DeribitBuilder;
use dotenv::dotenv;
use env_logger::init;
use failure::Error;
use futures::compat::Compat;
use futures::{FutureExt, StreamExt};
use tokio::runtime::Runtime;

fn main() -> Result<()> {
    init();
    dotenv().unwrap();

    let drb = DeribitBuilder::default()
        .testnet(true)
        .sub_chan_size(10usize)
        .build()
        .unwrap();

    let mut rt = Runtime::new()?;

    let fut = async move {
        let (mut client, mut subscription) = await!(drb.connect())?;
        let req = SetHeartbeatRequest { interval: 10 };

        let resp = await!(client.public_set_heartbeat(req))?;
        println!("Hearbet response {:?}", resp);

        while let Some(sub) = await!(subscription.next()) {
            match sub {
                Either::Right(l) => match l.params.r#type {
                    HeartbeatType::TestRequest => {
                        println!("Test Requested");
                        await!(client.public_test(TestRequest::default()))?;
                    }
                    _ => println!("Heartbeat"),
                },
                _ => {}
            }
        }

        Ok::<_, Error>(())
    };

    let fut = Compat::new(fut.boxed());
    let r = rt.block_on(fut);
    println!("{:?}", r);
    Ok(())
}
