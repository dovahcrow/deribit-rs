#![feature(async_await)]

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
        let (mut client, mut subscription) = drb.connect().await?;
        let req = AuthRequest::credential_auth(&key, &secret);

        let _ = client.call(req).await?;
        let req = GetPositionsRequest {
            currency: Currency::BTC,
            ..Default::default()
        };
        let positions = client.call(req).await?;
        println!("{:?}", positions.await?);
        let req = PrivateSubscribeRequest {
            channels: vec![
                "user.portfolio.BTC".into(),
                "user.trades.BTC-PERPETUAL.raw".into(),
                "user.trades.BTC-28JUN19-3000-P.raw".into(),
            ],
        };

        let result = client.call(req).await?;
        println!("Subscription result: {:?}", result.await?);

        while let Some(sub) = subscription.next().await {
            println!("{:?}", sub);
        }

        Ok::<_, Error>(())
    };

    let fut = fut.boxed().compat();
    let r = rt.block_on(fut);
    println!("{:?}", r);
    Ok(())
}
