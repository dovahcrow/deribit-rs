#![feature(async_await, await_macro)]

use deribit::models::{AuthRequest, Currency, GetPositionsRequest, PrivateSubscribeRequest};
use deribit::DeribitBuilder;
use dotenv::dotenv;
use env_logger::init;
use failure::{Error, Fallible};
use futures::{FutureExt, StreamExt, TryFutureExt};
use std::env::var;
use tokio::runtime::Runtime;

fn main() -> Fallible<()> {
    let _ = dotenv();
    init();

    let key = var("DERIBIT_KEY").unwrap();
    let secret = var("DERIBIT_SECRET").unwrap();

    let drb = DeribitBuilder::default().testnet(true).build().unwrap();

    let mut rt = Runtime::new()?;

    let fut = async move {
        let (mut client, mut subscription) = await!(drb.connect())?;
        let req = AuthRequest::credential_auth(&key, &secret);

        let _ = await!(client.call(req))?;
        let req = GetPositionsRequest {
            currency: Currency::BTC,
            ..Default::default()
        };
        let positions = await!(client.call(req))?;
        println!("{:?}", await!(positions)?);
        let req = PrivateSubscribeRequest {
            channels: vec![
                "user.portfolio.BTC".into(),
                "user.trades.BTC-PERPETUAL.raw".into(),
                "user.trades.BTC-28JUN19-3000-P.raw".into(),
            ],
        };

        let result = await!(client.call(req))?;
        println!("Subscription result: {:?}", await!(result)?);

        while let Some(sub) = await!(subscription.next()) {
            println!("{:?}", sub);
        }

        Ok::<_, Error>(())
    };

    let fut = fut.boxed().compat();
    let r = rt.block_on(fut);
    println!("{:?}", r);
    Ok(())
}
