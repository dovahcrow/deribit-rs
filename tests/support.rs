use deribit::models::{GetTimeRequest, HelloRequest, TestRequest};
use deribit::{Deribit, DeribitError};
use fehler::throws;
use tokio::runtime::Runtime;

#[test]
#[throws(DeribitError)]
fn hello() {
    let drb = Deribit::new();
    let mut rt = Runtime::new().expect("cannot create tokio runtime");

    let fut = async {
        let (mut client, _) = drb.connect().await?;

        let req = HelloRequest {
            client_name: "deribit-rs".into(),
            client_version: "0.0.1".into(),
        };

        let _ = client.call(req).await?.await?;

        Ok::<_, DeribitError>(())
    };
    rt.block_on(fut)?;
}

#[test]
#[throws(DeribitError)]
fn get_time() {
    let drb = Deribit::new();
    let mut rt = Runtime::new().expect("cannot create tokio runtime");

    let fut = async {
        let (mut client, _) = drb.connect().await?;

        let _ = client.call(GetTimeRequest).await?.await;

        Ok::<_, DeribitError>(())
    };
    rt.block_on(fut)?;
}

#[test]
#[throws(DeribitError)]
fn test() {
    let drb = Deribit::new();
    let mut rt = Runtime::new().expect("cannot create tokio runtime");

    let fut = async {
        let (mut client, _) = drb.connect().await?;
        let req = TestRequest {
            expected_result: Some("exception".into()),
        };
        Ok::<_, DeribitError>(client.call(req).await?.await?)
    };
    assert!(rt.block_on(fut).is_err());
}
