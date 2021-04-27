use deribit::models::{AuthRequest, Currency, GetAccountSummaryRequest, GetSubaccountsRequest};
use deribit::{Deribit, DeribitBuilder};
use dotenv::dotenv;
use failure::Error;
use fehler::throw;
use fehler::throws;
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
        let _ = env_logger::try_init();
        Self {
            key: var("DERIBIT_KEY").unwrap(),
            secret: var("DERIBIT_SECRET").unwrap(),
            drb: DeribitBuilder::default().testnet(true).build().unwrap(),
            rt: Runtime::new().unwrap(),
        }
    }
}

#[test]
#[throws(Error)]
fn get_account_summary() {
    let AccountTest {
        rt,
        drb,
        key,
        secret,
    } = AccountTest::default();
    let fut = async move {
        let (mut client, _) = drb.connect().await?;
        let req = AuthRequest::credential_auth(&key, &secret);
        let _ = client.call(req).await?.await?;
        let req = GetAccountSummaryRequest::extended(Currency::BTC);
        Ok::<_, Error>(client.call(req).await?.await?)
    };
    let resp = rt.block_on(fut);
    if let Err(err) = resp {
        println!("{:?}", err);
        throw!(err);
    }
}

#[test]
#[throws(Error)]
fn get_subaccounts() {
    let AccountTest {
        rt,
        drb,
        key,
        secret,
    } = AccountTest::default();
    let fut = async move {
        let (mut client, _) = drb.connect().await?;
        let req = AuthRequest::credential_auth(&key, &secret);
        let _ = client.call(req).await?.await?;

        let req = GetSubaccountsRequest::with_portfolio();
        Ok::<_, Error>(client.call(req).await?.await?)
    };
    let resp = rt.block_on(fut);
    if let Err(err) = resp {
        println!("{:?}", err);
        throw!(err);
    }
}
