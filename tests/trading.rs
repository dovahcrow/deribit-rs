use anyhow::Error;
use deribit::models::{
    AuthRequest, BuyRequest, CancelByLabelRequest, CancelRequest, Currency, EditRequest,
    GetOpenOrdersByCurrencyRequest, GetOpenOrdersByInstrumentRequest, GetOrderStateRequest,
    SellRequest,
};
use deribit::DeribitBuilder;
use dotenv::dotenv;
use fehler::throws;
use std::env::var;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::time::sleep;

#[test]
#[throws(Error)]
fn get_order_state() {
    let _ = dotenv();
    let key = var("DERIBIT_KEY").unwrap();
    let secret = var("DERIBIT_SECRET").unwrap();

    let drb = DeribitBuilder::default().testnet(true).build().unwrap();
    let rt = Runtime::new().expect("cannot create tokio runtime");

    let fut = async move {
        let (mut client, _) = drb.connect().await?;
        let req = AuthRequest::credential_auth(&key, &secret);
        let _ = client.call(req).await?.await?;

        let req = GetOrderStateRequest::new("5703080407");
        Ok::<_, Error>(client.call(req).await?.await?)
    };
    let _ = rt.block_on(fut)?;
}

#[test]
#[throws(Error)]
fn buy_and_sell() {
    let _ = dotenv();

    let key = var("DERIBIT_KEY").unwrap();
    let secret = var("DERIBIT_SECRET").unwrap();
    let drb = DeribitBuilder::default().testnet(true).build().unwrap();
    let rt = Runtime::new().expect("cannot create tokio runtime");

    let fut = async move {
        let (mut client, _) = drb.connect().await?;
        let req = AuthRequest::credential_auth(&key, &secret);
        let _ = client.call(req).await?.await?;

        client
            .call(BuyRequest::market("BTC-PERPETUAL", 10.))
            .await?
            .await?;
        sleep(Duration::from_secs(1)).await;

        client
            .call(SellRequest::market("BTC-PERPETUAL", 10.))
            .await?
            .await?;
        Ok::<_, Error>(())
    };
    let _ = rt.block_on(fut)?;
}

#[test]
#[throws(Error)]
fn buy_and_edit_and_inspect_and_cancel() {
    let _ = dotenv();

    let key = var("DERIBIT_KEY").unwrap();
    let secret = var("DERIBIT_SECRET").unwrap();
    let drb = DeribitBuilder::default().testnet(true).build().unwrap();
    let rt = Runtime::new().expect("cannot create tokio runtime");

    let fut = async move {
        let (mut client, _) = drb.connect().await?;
        let req = AuthRequest::credential_auth(&key, &secret);
        let _ = client.call(req).await?.await?;

        let id = client
            .call(BuyRequest::limit("BTC-PERPETUAL", 1000., 10.))
            .await?
            .await?
            .0
            .order
            .order_id;

        client
            .call(EditRequest::new(&id, 1200., 10.))
            .await?
            .await?;
        client
            .call(GetOpenOrdersByCurrencyRequest::by_currency(Currency::BTC))
            .await?
            .await?;
        client
            .call(GetOpenOrdersByInstrumentRequest::by_instrument(
                "BTC-PERPETUAL",
            ))
            .await?
            .await?;
        client.call(CancelRequest::new(&id)).await?.await?;
        Ok::<_, Error>(())
    };
    let _ = rt.block_on(fut)?;
}

#[test]
#[throws(Error)]
fn buy_and_cancel_by_label() {
    let _ = dotenv();

    let key = var("DERIBIT_KEY").unwrap();
    let secret = var("DERIBIT_SECRET").unwrap();
    let drb = DeribitBuilder::default().testnet(true).build().unwrap();
    let rt = Runtime::new().expect("cannot create tokio runtime");

    let fut = async move {
        let (mut client, _) = drb.connect().await?;
        let req = AuthRequest::credential_auth(&key, &secret);
        let _ = client.call(req).await?.await?;

        let mut req = BuyRequest::limit("BTC-PERPETUAL", 1000., 10.);
        req.label = Some("happy".to_string());

        client.call(req).await?.await?.0.order.order_id;

        client
            .call(CancelByLabelRequest::new("happy"))
            .await?
            .await?;
        Ok::<_, Error>(())
    };
    let _ = rt.block_on(fut)?;
}
