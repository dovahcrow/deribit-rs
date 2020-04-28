use deribit::models::{
    HeartbeatType, PublicSubscribeRequest, SetHeartbeatRequest, SubscriptionParams, TestRequest,
};
use deribit::DeribitBuilder;
use dotenv::dotenv;
use env_logger::init;
use failure::Error;
use fehler::throws;
use futures::StreamExt;

#[throws(Error)]
#[tokio::main]
async fn main() {
    let _ = dotenv();
    init();

    let drb = DeribitBuilder::default()
        .subscription_buffer_size(100000usize)
        .build()
        .unwrap();

    let (mut client, mut subscription) = drb.connect().await?;

    let req = PublicSubscribeRequest::new(&[
        "book.BTC-PERPETUAL.5.20.100ms".into(),
        "book.BTC-PERPETUAL.raw".into(),
        "deribit_price_index.eth_usd".into(),
        "deribit_price_ranking.eth_usd".into(),
        "estimated_expiration_price.btc_usd".into(),
        "markprice.options.btc_usd".into(),
        "perpetual.BTC-PERPETUAL.raw".into(),
        "quote.BTC-PERPETUAL".into(),
        "ticker.BTC-PERPETUAL.raw".into(),
        "trades.BTC-PERPETUAL.raw".into(),
    ]);

    let _ = client.call(req).await?.await?;

    client
        .call(SetHeartbeatRequest::with_interval(30))
        .await?
        .await?;

    while let Some(m) = subscription.next().await {
        if let SubscriptionParams::Heartbeat {
            r#type: HeartbeatType::TestRequest,
        } = m?.params
        {
            client.call(TestRequest::default()).await?.await?;
        }
    }
}
