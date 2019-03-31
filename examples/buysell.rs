#![feature(async_await, futures_api, await_macro)]

use deribit::errors::Result;
use deribit::models::{AuthRequest, BuyRequest, SellRequest};
use deribit::Deribit;
use dotenv::dotenv;
use env_logger::init;
use failure::Error;
use futures::compat::Compat;
use futures::FutureExt;
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
        let (mut client, _) = await!(drb.connect())?;
        let req = AuthRequest::credential_auth(&key, &secret);

        let _ = await!(client.public_auth(req))?;
        let req = BuyRequest::market("BTC-PERPETUAL", 10f64);
        let resp = await!(client.private_buy(req))?;
        println!("{:?}", resp);
        let req = SellRequest::market("BTC-PERPETUAL", 10f64);
        let resp = await!(client.private_sell(req))?;
        println!("{:?}", resp);

        Ok::<_, Error>(())
    };

    let fut = Compat::new(fut.boxed());
    let r = rt.block_on(fut);
    println!("{:?}", r);
    Ok(())
}
