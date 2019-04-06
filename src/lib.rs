#![feature(futures_api, async_await, await_macro, specialization)]
#![recursion_limit = "512"]

mod api_client;
pub mod errors;
pub mod models;
mod subscription_client;

pub use crate::api_client::{DeribitAPICallResult, DeribitAPIClient};
pub use crate::subscription_client::DeribitSubscriptionClient;

use crate::errors::Result;
use crate::models::{Either, HeartbeatMessage, JSONRPCResponse, SubscriptionMessage, WSMessage};
use derive_builder::Builder;
use futures::channel::{mpsc, oneshot};
use futures::compat::{Compat, Future01CompatExt, Sink01CompatExt, Stream01CompatExt};
use futures::{select, FutureExt, SinkExt, Stream, StreamExt, TryFutureExt, TryStreamExt};
use futures01::Stream as Stream01;
use log::warn;
use log::{error, info, trace};
use serde_json::from_str;
use std::collections::HashMap;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::timer::Timeout;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::Message;
use url::Url;

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub const WS_URL: &'static str = "wss://www.deribit.com/ws/api/v2";
pub const WS_URL_TESTNET: &'static str = "wss://test.deribit.com/ws/api/v2";

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
pub struct Deribit {
    #[builder(default)]
    testnet: bool,
    #[builder(default = "10")]
    sub_chan_size: usize,
}

impl Deribit {
    pub fn new() -> Deribit {
        DeribitBuilder::default().build().unwrap()
    }

    pub async fn connect(self) -> Result<(DeribitAPIClient, DeribitSubscriptionClient)> {
        let ws_url = if self.testnet { WS_URL_TESTNET } else { WS_URL };
        info!("Connecting");
        let (ws, _) = await!(connect_async(Url::parse(ws_url)?).compat())?;

        let (wstx, wsrx) = ws.split();
        let (stx, srx) = mpsc::channel(100000);
        let (waiter_tx, waiter_rx) = mpsc::channel(10);
        let back = Self::servo(wsrx.compat().err_into(), waiter_rx, stx).map_err(|e| {
            error!("[Servo] Exiting because of '{}'", e);
        });
        let back = back.boxed();

        tokio::spawn(Compat::new(back));

        Ok((
            DeribitAPIClient::new(wstx.sink_compat(), waiter_tx),
            DeribitSubscriptionClient::new(srx),
        ))
    }

    pub async fn servo(
        ws: impl Stream<Item = Result<Message>> + Unpin,
        mut waiter_rx: mpsc::Receiver<(i64, oneshot::Sender<Result<JSONRPCResponse>>)>,
        mut stx: mpsc::Sender<Either<SubscriptionMessage, HeartbeatMessage>>,
    ) -> Result<()> {
        let mut ws = ws.fuse();
        let mut waiters: HashMap<i64, oneshot::Sender<Result<JSONRPCResponse>>> = HashMap::new();

        loop {
            select! {
                msg = ws.next() => {
                    trace!("[Servo] Message: {:?}", msg);

                    match msg.unwrap()? {
                        Message::Text(msg) => {
                            let resp: WSMessage = match from_str(&msg) {
                                Ok(msg) => msg,
                                Err(e) => {
                                    error!("Cannot decode rpc message {:?}", e);
                                    Err(e)?
                                }
                            };

                            match resp {
                                WSMessage::RPC(msg) => {
                                    let id = msg.id;
                                    if let Err(_) = waiters.remove(&msg.id).unwrap().send(msg.to_result()) {
                                        warn!("The client for {} is dropped", id);
                                    }
                                }
                                WSMessage::Subscription(event) => {
                                    let fut = Compat::new(stx.send(Either::Left(event)));
                                    let fut = Timeout::new(fut, Duration::from_millis(1)).compat();
                                    await!(fut)?
                                },
                                 WSMessage::Heartbeat(event) => {
                                    let fut = Compat::new(stx.send(Either::Right(event)));
                                    let fut = Timeout::new(fut, Duration::from_millis(1)).compat();
                                    await!(fut)?
                                },
                            };
                        }
                        Message::Ping(_) => {
                            println!("Received Ping");
                        }
                        Message::Pong(_) => {
                            println!("Received Ping");
                        }
                        Message::Binary(_) => {
                            println!("Received Binary");
                        }
                    }
                }

                waiter = waiter_rx.next() => {
                    if let Some((id, waiter)) = waiter {
                        waiters.insert(id, waiter);
                    } else {
                        warn!("[Servo] API Client dropped");
                    }
                }
            };
        }
    }
}
