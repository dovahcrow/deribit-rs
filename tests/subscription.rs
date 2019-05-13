#![feature(async_await)]

use deribit::models::subscription::{PrivateSubscribeRequest, PublicSubscribeRequest};
use deribit::models::{AuthRequest, BuyRequest, CancelRequest, SellRequest};
use deribit::{Deribit, DeribitBuilder};
use dotenv::dotenv;
use failure::{Error, Fallible};
use fluid::prelude::*;
use futures::{FutureExt, StreamExt, TryFutureExt};
use std::env::var;
use tokio::runtime::Runtime;


struct SubscriptionTest {
    rt: Runtime,
    drb: Deribit,
    key: String,
    secret: String,
}

impl Default for SubscriptionTest {
    fn default() -> Self {
        let _ = dotenv();
        // env_logger::init();
        Self {
            key: var("DERIBIT_KEY").unwrap(),
            secret: var("DERIBIT_SECRET").unwrap(),
            drb: DeribitBuilder::default().testnet(true).build().unwrap(),
            rt: Runtime::new().unwrap(),

        }
    }
}


// The tests:
#[session]
impl SubscriptionTest {
    // #[fact]
    // fn announcements(self) -> Fallible<()> {
    //     let Self { drb, mut rt, .. } = self;
    //     let fut = async {
    //         let (mut client, subscription) = drb.connect().await.unwrap();

    //         let req = PrivateSubscribeRequest::new(&["announcements".into()]);
    //         let _ = client.call(req).await.unwrap();

    //         let v = subscription.take(1).collect::<Vec<_>>().await;
    //         Ok::<_, Error>(v)
    //     };

    //     let fut = fut.boxed().compat();
    //     let v = rt.block_on(fut)?;
    //     v.len().should().be_equal_to(1);
    //     Ok(())
    // }

    #[fact]
    fn book(self) -> Fallible<()> {
        let Self { drb, mut rt, .. } = self;

        let fut = async {
            let (mut client, subscription) = drb.connect().await.unwrap();

            let req = PublicSubscribeRequest::new(&[
                "book.BTC-PERPETUAL.raw".into(),
                "book.ETH-PERPETUAL.raw".into(),
            ]);

            let _ = client.call(req).await.unwrap();

            let v = subscription.take(5).collect::<Vec<_>>().await;
            Ok::<_, Error>(v)
        };

        let fut = fut.boxed().compat();
        let v = rt.block_on(fut)?;
        v.len().should().be_equal_to(5);
        Ok(())
    }

    #[fact]
    fn grouped_book(self) -> Fallible<()> {
        let Self { drb, mut rt, .. } = self;

        let fut = async {
            let (mut client, subscription) = drb.connect().await.unwrap();

            let req = PublicSubscribeRequest::new(&[
                "book.BTC-PERPETUAL.10.20.100ms".into(),
                "book.ETH-PERPETUAL.10.20.100ms".into(),
            ]);

            let _ = client.call(req).await.unwrap();

            let v = subscription.take(5).collect::<Vec<_>>().await;
            Ok::<_, Error>(v)
        };

        let fut = fut.boxed().compat();
        let v = rt.block_on(fut)?;
        v.len().should().be_equal_to(5);
        Ok(())
    }


    #[fact]
    fn deribit_price_index(self) -> Fallible<()> {
        let Self { drb, mut rt, .. } = self;
        let fut = async {
            let (mut client, subscription) = drb.connect().await.unwrap();

            let req = PublicSubscribeRequest::new(&[
                "deribit_price_index.btc_usd".into(),
                "deribit_price_index.eth_usd".into(),
            ]);
            let _ = client.call(req).await.unwrap();

            let v = subscription.take(2).collect::<Vec<_>>().await;
            Ok::<_, Error>(v)
        };

        let fut = fut.boxed().compat();
        let v = rt.block_on(fut)?;
        v.len().should().be_equal_to(2);
        Ok(())
    }


    #[fact]
    fn deribit_price_ranking(self) -> Fallible<()> {
        let Self { drb, mut rt, .. } = self;
        let fut = async {
            let (mut client, subscription) = drb.connect().await.unwrap();

            let req = PublicSubscribeRequest::new(&[
                "deribit_price_ranking.btc_usd".into(),
                "deribit_price_ranking.eth_usd".into(),
            ]);
            let _ = client.call(req).await.unwrap();

            let v = subscription.take(2).collect::<Vec<_>>().await;
            Ok::<_, Error>(v)
        };

        let fut = fut.boxed().compat();
        let v = rt.block_on(fut)?;
        v.len().should().be_equal_to(2);
        Ok(())
    }

    #[fact]
    fn estimated_expiration_price(self) -> Fallible<()> {
        let Self { drb, mut rt, .. } = self;
        let fut = async {
            let (mut client, subscription) = drb.connect().await.unwrap();

            let req = PublicSubscribeRequest::new(&[
                "estimated_expiration_price.btc_usd".into(),
                "estimated_expiration_price.eth_usd".into(),
            ]);
            let _ = client.call(req).await.unwrap();

            let v = subscription.take(2).collect::<Vec<_>>().await;
            Ok::<_, Error>(v)
        };

        let fut = fut.boxed().compat();
        let v = rt.block_on(fut)?;
        v.len().should().be_equal_to(2);
        Ok(())
    }

    #[fact]
    fn markprice_options(self) -> Fallible<()> {
        let Self { drb, mut rt, .. } = self;
        let fut = async {
            let (mut client, subscription) = drb.connect().await.unwrap();

            let req = PublicSubscribeRequest::new(&[
                "markprice.options.btc_usd".into(),
                "markprice.options.eth_usd".into(),
            ]);
            let _ = client.call(req).await.unwrap();

            let v = subscription.take(2).collect::<Vec<_>>().await;
            Ok::<_, Error>(v)
        };

        let fut = fut.boxed().compat();
        let v = rt.block_on(fut)?;
        v.len().should().be_equal_to(2);
        Ok(())
    }

    #[fact]
    fn perpetual(self) -> Fallible<()> {
        let Self { drb, mut rt, .. } = self;
        let fut = async {
            let (mut client, subscription) = drb.connect().await.unwrap();

            let req = PublicSubscribeRequest::new(&[
                "perpetual.BTC-PERPETUAL.raw".into(),
                "perpetual.ETH-PERPETUAL.raw".into(),
            ]);
            let _ = client.call(req).await.unwrap();

            let v = subscription.take(2).collect::<Vec<_>>().await;
            Ok::<_, Error>(v)
        };

        let fut = fut.boxed().compat();
        let v = rt.block_on(fut)?;
        v.len().should().be_equal_to(2);
        Ok(())
    }

    #[fact]
    fn quote(self) -> Fallible<()> {
        let Self { drb, mut rt, .. } = self;
        let fut = async {
            let (mut client, subscription) = drb.connect().await.unwrap();

            let req = PublicSubscribeRequest::new(&[
                "quote.BTC-PERPETUAL".into(),
                "quote.ETH-PERPETUAL".into(),
            ]);
            let _ = client.call(req).await.unwrap();

            let v = subscription.take(10).collect::<Vec<_>>().await;
            Ok::<_, Error>(v)
        };

        let fut = fut.boxed().compat();
        let v = rt.block_on(fut)?;
        v.len().should().be_equal_to(10);
        Ok(())
    }

    #[fact]
    fn ticker(self) -> Fallible<()> {
        let Self { drb, mut rt, .. } = self;
        let fut = async {
            let (mut client, subscription) = drb.connect().await.unwrap();

            let req = PublicSubscribeRequest {
                channels: vec![
                    "ticker.BTC-PERPETUAL.raw".into(),
                    "ticker.ETH-PERPETUAL.raw".into(),
                    "ticker.BTC-28JUN19.100ms".into(),
                    "ticker.BTC-28JUN19.raw".into(),
                    "ticker.BTC-28JUN19-7500-P.raw".into(),
                    "ticker.BTC-28JUN19-7500-P.100ms".into(),
                ],
            };

            let _ = client.call(req).await.unwrap();

            let v = subscription.take(5).collect::<Vec<_>>().await;

            Ok::<Vec<_>, Error>(v)
        };

        let fut = fut.boxed().compat();
        let v = rt.block_on(fut)?;
        v.len().should().be_equal_to(5);
        Ok(())
    }


    #[fact]
    fn trades(self) -> Fallible<()> {
        let Self {
            mut rt,
            drb,
            key,
            secret,
        } = self;

        let fut = async move {
            let (mut client, subscription) = drb.connect().await.unwrap();
            let _ = client
                .call(AuthRequest::credential_auth(&key, &secret))
                .await?;

            let req = PublicSubscribeRequest {
                channels: vec![
                    "trades.BTC-PERPETUAL.raw".into(),
                    "trades.ETH-PERPETUAL.raw".into(),
                ],
            };
            let _ = client.call(req).await.unwrap();

            client
                .call(BuyRequest::market("BTC-PERPETUAL", 10.))
                .await?
                .await?;

            client
                .call(SellRequest::market("BTC-PERPETUAL", 10.))
                .await?
                .await?;
            let v = subscription.take(2).collect::<Vec<_>>().await;
            Ok::<_, Error>(v)
        };

        let fut = fut.boxed().compat();
        let v = rt.block_on(fut)?;
        v.len().should().be_equal_to(2);
        Ok(())
    }


    #[fact]
    fn user_orders(self) -> Fallible<()> {
        let Self {
            mut rt,
            drb,
            key,
            secret,
        } = self;

        let fut = async move {
            let (mut client, subscription) = drb.connect().await.unwrap();

            let _ = client
                .call(AuthRequest::credential_auth(&key, &secret))
                .await?;

            let req = PrivateSubscribeRequest {
                channels: vec!["user.orders.BTC-PERPETUAL.raw".into()],
            };
            let _ = client.call(req).await.unwrap();

            let req = BuyRequest::limit("BTC-PERPETUAL", 10f64, 10f64);

            let resp = client.call(req).await?.await?;
            let id = resp.0.order.order_id;

            let v = subscription.take(1).collect::<Vec<_>>().await;
            let req = CancelRequest::new(&id);
            let resp = client.call(req).await?.await?;
            id.should().be_equal_to(resp.order.order_id);
            Ok::<_, Error>(v)
        };

        let fut = fut.boxed().compat();
        let _ = rt.block_on(fut)?;
        Ok(())
    }

    #[fact]
    fn user_portfolio(self) -> Fallible<()> {
        let Self {
            mut rt,
            drb,
            key,
            secret,
        } = self;

        let fut = async move {
            let (mut client, subscription) = drb.connect().await.unwrap();

            let _ = client
                .call(AuthRequest::credential_auth(&key, &secret))
                .await?;

            let req = PrivateSubscribeRequest::new(&[
                "user.portfolio.BTC".into(),
                "user.portfolio.ETH".into(),
            ]);
            let _ = client.call(req).await.unwrap();

            let v = subscription.take(2).collect::<Vec<_>>().await;
            Ok::<_, Error>(v)
        };

        let fut = fut.boxed().compat();
        let v = rt.block_on(fut)?;
        v.len().should().be_equal_to(2);
        Ok(())
    }
}