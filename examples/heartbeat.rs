#![feature(async_await)]

use deribit::models::{Either, HeartbeatType, SetHeartbeatRequest, TestRequest};
use deribit::DeribitBuilder;
use dotenv::dotenv;
use env_logger::init;
use failure::{Error, Fallible};
use futures::{FutureExt, StreamExt, TryFutureExt};
use tokio::runtime::Runtime;

fn main() -> Fallible<()> {
    dotenv().unwrap();
    init();

    let drb = DeribitBuilder::default().testnet(true).build().unwrap();

    let mut rt = Runtime::new()?;

    let fut = async move {
        let (mut client, mut subscription) = drb.connect().await?;

        let resp = client.call(SetHeartbeatRequest::with_interval(10)).await?;
        println!("Hearbet response {:?}", resp.await?);

        while let Some(sub) = subscription.next().await {
            match sub {
                Either::Right(l) => match l.params.r#type {
                    HeartbeatType::TestRequest => {
                        println!("Test Requested");
                        client.call(TestRequest::default()).await?;
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
