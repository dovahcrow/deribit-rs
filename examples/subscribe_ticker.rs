#![feature(async_await, futures_api, await_macro)]

use deribit::models::PublicSubscribeRequest;
use deribit::Deribit;
use env_logger::init;
use futures::{FutureExt, StreamExt, TryFutureExt};
use log::info;

fn main() {
    dotenv::dotenv().unwrap();
    init();

    let drb = Deribit::new();
    let fut = async {
        let (mut client, mut subscription) = await!(drb.connect()).unwrap();

        let req = PublicSubscribeRequest {
            channels: vec!["ticker.BTC-28JUN19.100ms".into()],
        };

        let _ = await!(client.call(req)).unwrap();

        info!("Successfully subscribed to tickers.BTC-PERPETUAL.raw");

        while let Some(sub) = await!(subscription.next()) {
            info!("{:?}", sub);
        }
    };
    let fut = fut.boxed().map(|_| Ok(())).compat();
    tokio::run(fut);
}
