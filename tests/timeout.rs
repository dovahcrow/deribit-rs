use anyhow::Error;
use deribit::{models::HelloRequest, Deribit};
use std::time::Duration;
use tokio::runtime::Runtime;

#[test]
#[should_panic]
fn timeout() {
    let drb = Deribit::builder()
        .timeout(Duration::from_millis(1))
        .build()
        .unwrap();

    let rt = Runtime::new().expect("cannot create tokio runtime");

    let fut = async {
        let (mut client, _) = drb.connect().await?;

        let req = HelloRequest {
            client_name: "deribit-rs".into(),
            client_version: "0.0.1".into(),
        };

        let _ = client.call(req).await?.await?;

        Ok::<_, Error>(())
    };
    rt.block_on(fut).unwrap();
}
