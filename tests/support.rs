use deribit::models::{GetTimeRequest, HelloRequest, TestRequest};
use deribit::Deribit;
use failure::{Error, Fallible};
use tokio::runtime::Runtime;

#[test]
fn hello() -> Fallible<()> {
    let drb = Deribit::new();
    let rt = Runtime::new()?;

    let fut = async {
        let (mut client, _) = drb.connect().await?;

        let req = HelloRequest {
            client_name: "deribit-rs".into(),
            client_version: "0.0.1".into(),
        };

        let _ = client.call(req).await?.await?;

        Ok::<_, Error>(())
    };
    rt.block_on(fut)?;
    Ok(())
}

#[test]
fn get_time() -> Fallible<()> {
    let drb = Deribit::new();
    let rt = Runtime::new()?;

    let fut = async {
        let (mut client, _) = drb.connect().await?;

        let _ = client.call(GetTimeRequest).await?.await;

        Ok::<_, Error>(())
    };
    rt.block_on(fut)?;
    Ok(())
}

#[test]
fn test() -> Fallible<()> {
    let drb = Deribit::new();
    let rt = Runtime::new()?;

    let fut = async {
        let (mut client, _) = drb.connect().await?;
        let req = TestRequest {
            expected_result: Some("exception".into()),
        };
        Ok::<_, Error>(client.call(req).await?.await?)
    };
    assert!(rt.block_on(fut).is_err());
    Ok(())
}
