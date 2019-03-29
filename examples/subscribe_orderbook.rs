#![feature(async_await, futures_api, await_macro)]

use deribit::models::SubscribeRequest;
use deribit::Deribit;
use env_logger::init;
use futures::compat::Compat;
use futures::{FutureExt, StreamExt};
use log::info;

fn main() {
    init();

    let drb = Deribit::new();
    let fut = async {
        let (mut client, mut subscription) = await!(drb.connect()).unwrap();

        let req = SubscribeRequest {
            channels: vec!["book.BTC-PERPETUAL.raw".into()],
        };

        let _ = await!(client.public_subscribe(&req)).unwrap();

        info!("Successfully subscribed to book.BTC-PERPETUAL.raw") ;

        while let Some(sub) = await!(subscription.next()) {
            info!("{:?}", sub);
        }
    };
    let fut = Compat::new(fut.boxed().map(|_| Ok(())));
    tokio::run(fut);
}
 