#![feature(async_await, futures_api, await_macro)]

use deribit::errors::Result;
use deribit::models::{AuthRequest, SubscribeRequest};
use deribit::Deribit;
use dotenv::dotenv;
use env_logger::init;
use failure::Error;
use futures::compat::Compat;
use futures::{FutureExt, StreamExt};
use std::env::var;
use tokio::runtime::Runtime;

fn main() -> Result<()> {
    init();
    dotenv().unwrap();

    let key = var("DERIBIT_KEY").unwrap();
    let secret = var("DERIBIT_SECRET").unwrap();

    let drb = Deribit::new_testnet();

    let mut rt = Runtime::new()?;

    let fut = async move {
        let (mut client, mut subscription) = await!(drb.connect())?;
        let req = AuthRequest::credential_auth(&key, &secret);

        let _ = await!(client.public_auth(&req))?;

        let req = SubscribeRequest {
            channels: vec!["user.portfolio.BTC".into()],
        };

        let result = await!(client.private_subscribe(&req))?;
        println!("Subscription result: {:?}", result);

        while let Some(sub) = await!(subscription.next()) {
            println!("{:?}", sub);
        }

        Ok::<_, Error>(())
    };

    let fut = Compat::new(fut.boxed());
    let r = rt.block_on(fut);
    println!("{:?}", r);
    Ok(())
}
