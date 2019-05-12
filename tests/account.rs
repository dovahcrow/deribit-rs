#![feature(async_await)]

use deribit::models::{AuthRequest, Currency, GetAccountSummaryRequest, GetSubaccountsRequest};
use deribit::{Deribit, DeribitBuilder};
use dotenv::dotenv;
use failure::{Error, Fallible};
use fluid::prelude::*;
use futures::{FutureExt, TryFutureExt};
use std::env::var;
use tokio::runtime::Runtime;


pub struct AccountTest {
    key: String,
    secret: String,
    drb: Deribit,
    rt: Runtime,

}

impl Default for AccountTest {
    fn default() -> Self {
        let _ = dotenv();
        Self {
            key: var("DERIBIT_KEY").unwrap(),
            secret: var("DERIBIT_SECRET").unwrap(),
            drb: DeribitBuilder::default().testnet(true).build().unwrap(),
            rt: Runtime::new().unwrap(),
        }
    }
}

#[session]
impl AccountTest {
    #[fact]
    fn get_account_summary(self) -> Fallible<()> {
        let Self {
            mut rt,
            drb,
            key,
            secret,
        } = self;
        let fut = async move {
            let (mut client, _) = drb.connect().await?;
            let req = AuthRequest::credential_auth(&key, &secret);
            let _ = client.call(req).await?.await?;
            let req = GetAccountSummaryRequest::extended(Currency::BTC);
            Ok::<_, Error>(client.call(req).await?.await?)
        };

        let fut = fut.boxed().compat();
        let _ = rt.block_on(fut)?;
        Ok(())
    }

    #[fact]
    fn get_subaccounts(self) -> Fallible<()> {
        let Self {
            mut rt,
            drb,
            key,
            secret,
        } = self;
        let fut = async move {
            let (mut client, _) = drb.connect().await?;
            let req = AuthRequest::credential_auth(&key, &secret);
            let _ = client.call(req).await?.await?;

            let req = GetSubaccountsRequest::with_portfolio();
            Ok::<_, Error>(client.call(req).await?.await?)
        };
        let fut = fut.boxed().compat();
        let _ = rt.block_on(fut)?;
        Ok(())
    }
}