#![feature(futures_api, async_await, await_macro)]
#![recursion_limit = "256"]

use crate::api_client::DeribitAPIClient;
use crate::errors::Result;
use crate::models::{SubscriptionMessage, WSMessage};
use crate::subscription_client::DeribitSubscriptionClient;
use futures::channel::{mpsc, oneshot};
use futures::compat::{Compat, Future01CompatExt, Sink01CompatExt, Stream01CompatExt};
use futures::{select, FutureExt, SinkExt, Stream, StreamExt, TryFutureExt, TryStreamExt};
use futures01::Stream as Stream01;
use log::{debug, error, info};
use serde_json::{from_str, Value};
use std::collections::HashMap;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::Message;
use url::Url;

mod api_client;
pub mod errors;
pub mod models;
mod subscription_client;

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

const WS_URL: &'static str = "wss://www.deribit.com/ws/api/v2";
const WS_URL_TESTNET: &'static str = "wss://test.deribit.com/ws/api/v2";

#[derive(Default)]
pub struct Deribit {
    testnet: bool,
}

impl Deribit {
    pub fn new() -> Deribit {
        Default::default()
    }

    pub fn new_testnet() -> Deribit {
        Deribit { testnet: true }
    }

    pub async fn connect(self) -> Result<(DeribitAPIClient, DeribitSubscriptionClient)> {
        let ws_url = if self.testnet { WS_URL_TESTNET } else { WS_URL };
        info!("Connecting");
        let (ws, _) = await!(connect_async(Url::parse(ws_url)?).compat())?;

        let (wstx, wsrx) = ws.split();
        let (stx, srx) = mpsc::channel(0);
        let (waiter_tx, waiter_rx) = mpsc::channel(0);
        let back = Self::servo(wsrx.compat().err_into(), waiter_rx, stx).map_err(|_| ());
        let back = back.boxed();

        tokio::spawn(Compat::new(back));

        Ok((DeribitAPIClient::new(wstx.sink_compat(), waiter_tx), DeribitSubscriptionClient::new(srx)))
    }

    pub async fn servo(
        ws: impl Stream<Item = Result<Message>> + Unpin,
        mut waiter_rx: mpsc::Receiver<(i64, oneshot::Sender<Result<Value>>)>,
        mut stx: mpsc::Sender<SubscriptionMessage>,
    ) -> Result<()> {
        let mut ws = ws.fuse();
        let mut waiters: HashMap<i64, oneshot::Sender<Result<Value>>> = HashMap::new();

        loop {
            select! {
                msg = ws.next() => {
                    debug!("[Servo] Message: {:?}", msg);
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
                                WSMessage::RPC(msg) => waiters.remove(&msg.id).unwrap().send(msg.to_result()).unwrap(),
                                WSMessage::Subscription(event) => await!(stx.send(event))?,
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
                        error!("[Servo] Waiter is none");
                    }
                }
            };
        }
    }
}
