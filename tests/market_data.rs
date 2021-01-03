use deribit::models::{
    Currency, GetBookSummaryByCurrencyRequest, GetIndexRequest, GetInstrumentsRequest,
};
use deribit::DeribitBuilder;
use dotenv::dotenv;
use failure::Error;
use fehler::throws;
use tokio::runtime::Runtime;

#[test]
#[throws(Error)]
fn get_index() {
    let _ = dotenv();
    let _ = env_logger::try_init();

    let drb = DeribitBuilder::default().testnet(true).build().unwrap();
    let rt = Runtime::new().expect("cannot create tokio runtime");

    let fut = async move {
        let (mut client, _) = drb.connect().await?;
        let req = GetIndexRequest::new(Currency::BTC);
        let _ = client.call(req).await?.await?;
        let req = GetIndexRequest::new(Currency::ETH);
        let _ = client.call(req).await?.await?;

        Ok::<_, Error>(())
    };
    let _ = rt.block_on(fut)?;
}

#[test]
#[throws(Error)]
fn get_instruments() {
    let _ = dotenv();
    let _ = env_logger::try_init();

    let drb = DeribitBuilder::default().build().unwrap();
    let rt = Runtime::new().expect("cannot create tokio runtime");

    let fut = async move {
        let (mut client, _) = drb.connect().await?;
        let req = GetInstrumentsRequest::new(Currency::BTC);
        let _ = client.call(req).await?.await?;
        let req = GetInstrumentsRequest::expired(Currency::ETH);
        let _ = client.call(req).await?.await?;

        Ok::<_, Error>(())
    };
    let _ = rt.block_on(fut)?;
}

#[test]
#[throws(Error)]
fn get_book_summary_by_currency() {
    let _ = dotenv();
    let _ = env_logger::try_init();
    let drb = DeribitBuilder::default().build().unwrap();
    let rt = Runtime::new().expect("cannot create tokio runtime");

    let fut = async move {
        let (mut client, _) = drb.connect().await?;
        let req = GetBookSummaryByCurrencyRequest::futures(Currency::BTC);
        let _ = client.call(req).await?.await?;
        let req = GetBookSummaryByCurrencyRequest::all(Currency::ETH);
        let _ = client.call(req).await?.await?;
        let req = GetBookSummaryByCurrencyRequest::options(Currency::ETH);
        let _ = client.call(req).await?.await?;

        Ok::<_, Error>(())
    };
    let _ = rt.block_on(fut)?;
}
