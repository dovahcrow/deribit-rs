use anyhow::Error;
use deribit::models::{HeartbeatType, SetHeartbeatRequest, SubscriptionParams, TestRequest};
use deribit::DeribitBuilder;
use dotenv::dotenv;
use env_logger::init;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = dotenv();
    init();

    let drb = DeribitBuilder::default().testnet(true).build().unwrap();

    let (mut client, mut subscription) = drb.connect().await?;

    let resp = client.call(SetHeartbeatRequest::with_interval(10)).await?;
    println!("Hearbet response {:?}", resp.await?);

    while let Some(Ok(sub)) = subscription.next().await {
        if sub.is_heartbeat() {
            match sub.params {
                SubscriptionParams::Heartbeat { r#type: ty } => match ty {
                    HeartbeatType::TestRequest => {
                        println!("Test Requested");
                        client.call(TestRequest::default()).await?;
                    }
                    _ => println!("Heartbeat"),
                },
                _ => {}
            }
        }
    }

    Ok(())
}
