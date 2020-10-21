use deribit::models::{AuthRequest, Currency, GetPositionsRequest, PrivateSubscribeRequest};
use deribit::DeribitBuilder;
use dotenv::dotenv;
use env_logger::init;
use failure::Error;
use futures::StreamExt;
use std::env::var;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = dotenv();
    init();

    let key = var("DERIBIT_KEY").unwrap();
    let secret = var("DERIBIT_SECRET").unwrap();

    let drb = DeribitBuilder::default().testnet(true).build().unwrap();

    let (mut client, mut subscription) = drb.connect().await?;

    let _ = client
        .call(AuthRequest::credential_auth(&key, &secret))
        .await?;

    let positions = client
        .call(GetPositionsRequest::futures(Currency::BTC))
        .await?
        .await?;

    println!("{:?}", positions);

    let req = PrivateSubscribeRequest::new(&[
        "user.portfolio.BTC".into(),
        "user.trades.BTC-PERPETUAL.raw".into(),
        "user.trades.BTC-28JUN19-3000-P.raw".into(),
    ]);

    let result = client.call(req).await?.await?;
    println!("Subscription result: {:?}", result);

    while let Some(sub) = subscription.next().await {
        println!("{:?}", sub);
    }

    Ok(())
}
