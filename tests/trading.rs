use deribit::models::{
    AuthRequest, BuyRequest, CancelRequest, Currency, EditRequest, GetOpenOrdersByCurrencyRequest,
    GetOpenOrdersByInstrumentRequest, GetOrderStateRequest, SellRequest,
};
use deribit::DeribitBuilder;
use dotenv::dotenv;
use failure::{Error, Fallible};
use fluid::prelude::*;
use std::env::var;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::time::delay_for;

struct TradingTest;

impl Default for TradingTest {
    fn default() -> Self {
        let _ = dotenv();
        TradingTest
    }
}

#[session]
impl TradingTest {
    #[fact]
    fn get_order_state(self) -> Fallible<()> {
        let key = var("DERIBIT_KEY").unwrap();
        let secret = var("DERIBIT_SECRET").unwrap();

        let drb = DeribitBuilder::default().testnet(true).build().unwrap();
        let mut rt = Runtime::new()?;

        let fut = async move {
            let (mut client, _) = drb.connect().await?;
            let req = AuthRequest::credential_auth(&key, &secret);
            let _ = client.call(req).await?.await?;

            let req = GetOrderStateRequest::new("2320198993");
            Ok::<_, Error>(client.call(req).await?.await?)
        };
        let _ = rt.block_on(fut)?;
        Ok(())
    }

    #[fact]
    fn buy_and_sell(self) -> Fallible<()> {
        let _ = dotenv();

        let key = var("DERIBIT_KEY").unwrap();
        let secret = var("DERIBIT_SECRET").unwrap();
        let drb = DeribitBuilder::default().testnet(true).build().unwrap();
        let mut rt = Runtime::new()?;

        let fut = async move {
            let (mut client, _) = drb.connect().await?;
            let req = AuthRequest::credential_auth(&key, &secret);
            let _ = client.call(req).await?.await?;

            client
                .call(BuyRequest::market("BTC-PERPETUAL", 10.))
                .await?
                .await?;
            delay_for(Duration::from_secs(1)).await;

            client
                .call(SellRequest::market("BTC-PERPETUAL", 10.))
                .await?
                .await?;
            Ok::<_, Error>(())
        };
        let _ = rt.block_on(fut)?;
        Ok(())
    }

    #[fact]
    fn buy_and_edit_and_inspect_and_cancel(self) -> Fallible<()> {
        let _ = dotenv();

        let key = var("DERIBIT_KEY").unwrap();
        let secret = var("DERIBIT_SECRET").unwrap();
        let drb = DeribitBuilder::default().testnet(true).build().unwrap();
        let mut rt = Runtime::new()?;

        let fut = async move {
            let (mut client, _) = drb.connect().await?;
            let req = AuthRequest::credential_auth(&key, &secret);
            let _ = client.call(req).await?.await?;

            let id = client
                .call(BuyRequest::limit("BTC-PERPETUAL", 10., 10.))
                .await?
                .await?
                .0
                .order
                .order_id;

            client.call(EditRequest::new(&id, 12., 10.)).await?.await?;
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
        Ok(())
    }
}
