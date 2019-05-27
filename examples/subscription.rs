#![feature(async_await)]

use deribit::models::subscription::{BookData, TickerData, TradesData};
use deribit::models::{
    Any3, HeartbeatType, PublicSubscribeRequest, SubscriptionParams, TestRequest,
};
use deribit::DeribitBuilder;
use dotenv::dotenv;
use env_logger::init;
use failure::Fallible;
use futures::StreamExt;
use runtime_tokio::Tokio;

#[runtime::main(Tokio)]
async fn main() -> Fallible<()> {
    let _ = dotenv();
    init();

    let drb = DeribitBuilder::default().testnet(true).build().unwrap();

    let (mut client, subscription) = drb.connect().await?;

    let mut subscription = subscription.limited::<Any3<BookData, TickerData, Vec<TradesData>>>();

    let req = PublicSubscribeRequest::new(&[
        "book.BTC-PERPETUAL.raw".into(),
        "book.BTC-28JUN19.raw".into(),
        "book.BTC-27SEP19.raw".into(),
        "ticker.BTC-PERPETUAL.raw".into(),
        "ticker.BTC-28JUN19.raw".into(),
        "ticker.BTC-27SEP19.raw".into(),
        "trades.BTC-PERPETUAL.raw".into(),
        "trades.BTC-28JUN19.raw".into(),
        "trades.BTC-27SEP19.raw".into(),
    ]);

    let _ = client.call(req).await?.await?;

    while let Some(m) = subscription.next().await {
        if let SubscriptionParams::Heartbeat {
            r#type: HeartbeatType::TestRequest,
        } = m?.params
        {
            client.call(TestRequest::default()).await?.await?;
        }
    }

    Ok(())
}
