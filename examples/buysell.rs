#![feature(async_await)]

use deribit::models::{AuthRequest, BuyRequest, SellRequest};
use deribit::DeribitBuilder;
use dotenv::dotenv;
use env_logger::init;
use failure::{Error, Fallible};
use futures::{FutureExt, TryFutureExt};
use std::env::var;
use tokio::runtime::Runtime;

fn main() -> Fallible<()> {
    dotenv().unwrap();
    init();

    let key = var("DERIBIT_KEY").unwrap();
    let secret = var("DERIBIT_SECRET").unwrap();

    let drb = DeribitBuilder::default().testnet(true).build().unwrap();

    let mut rt = Runtime::new()?;

    let fut = async move {
        let (mut client, _) = drb.connect().await?;
        let req = AuthRequest::credential_auth(&key, &secret);

        let _ = client.call(req).await?;
        let req = BuyRequest::market("BTC-PERPETUAL", 10f64);
        let resp = client.call(req).await?;
        println!("{:?}", resp.await?);
        let req = SellRequest::market("BTC-PERPETUAL", 10f64);
        let resp = client.call(req).await?;
        println!("{:?}", resp.await?);

        Ok::<_, Error>(())
    };

    let fut = fut.boxed().compat();
    let r = rt.block_on(fut);
    println!("{:?}", r);
    Ok(())
}
