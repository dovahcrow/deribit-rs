#![feature(async_await, await_macro, specialization)]
#![recursion_limit = "512"]

mod api_client;
pub mod errors;
pub mod models;
mod subscription_client;

pub use crate::api_client::{DeribitAPICallResult, DeribitAPIClient};
pub use crate::subscription_client::DeribitSubscriptionClient;

use crate::errors::DeribitError;
use crate::models::{Either, HeartbeatMessage, JSONRPCResponse, SubscriptionMessage, WSMessage};
use derive_builder::Builder;
use failure::Fallible;
use futures::channel::{mpsc, oneshot};
use futures::compat::{Future01CompatExt, Sink01CompatExt, Stream01CompatExt};
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
    subscription_buffer_size: usize,
}

impl Deribit {
    pub fn new() -> Deribit {
        DeribitBuilder::default().build().unwrap()
    }

    pub async fn connect(self) -> Fallible<(DeribitAPIClient, DeribitSubscriptionClient)> {
        let ws_url = if self.testnet { WS_URL_TESTNET } else { WS_URL };
        info!("Connecting");
        let (ws, _) = await!(connect_async(Url::parse(ws_url)?).compat())?;

        let (wstx, wsrx) = ws.split();
        let (stx, srx) = mpsc::channel(self.subscription_buffer_size);
        let (waiter_tx, waiter_rx) = mpsc::channel(10);
        let background = Self::servo(wsrx.compat().err_into(), waiter_rx, stx).map_err(|e| {
            error!("[Servo] Exiting because of '{}'", e);
        });

        tokio::spawn(background.boxed().compat());

        Ok((
            DeribitAPIClient::new(wstx.sink_compat(), waiter_tx),
            DeribitSubscriptionClient::new(srx),
        ))
    }

    pub async fn servo(
        ws: impl Stream<Item = Fallible<Message>> + Unpin,
        mut waiter_rx: mpsc::Receiver<(i64, oneshot::Sender<Fallible<JSONRPCResponse>>)>,
        mut stx: mpsc::Sender<Either<SubscriptionMessage, HeartbeatMessage>>,
    ) -> Fallible<()> {
        let mut ws = ws.fuse();
        let mut waiters: HashMap<i64, oneshot::Sender<Fallible<JSONRPCResponse>>> = HashMap::new();

        let mut orphan_messages = HashMap::new();
        loop {
            select! {
                msg = ws.next() => {
                    trace!("[Servo] Message: {:?}", msg);
                    let msg = if let Some(msg) = msg {
                        msg
                    } else {
                        break Err(DeribitError::WebsocketDisconnected)?;
                    };

                    match msg? {
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
                                    let waiter = match waiters.remove(&msg.id) {
                                        Some(waiter) => waiter,
                                        None => {
                                            orphan_messages.insert(msg.id, msg);
                                            continue;
                                        }
                                    };

                                    if let Err(msg) = waiter.send(msg.to_result()) {
                                        info!("The client for request {} is dropped, response is {:?}", id, msg);
                                    }
                                }
                                WSMessage::Subscription(event) => {
                                    let fut = stx.send(Either::Left(event)).compat();
                                    let fut = Timeout::new(fut, Duration::from_millis(1)).compat();
                                    await!(fut)?
                                }
                                WSMessage::Heartbeat(event) => {
                                    let fut = stx.send(Either::Right(event)).compat();
                                    let fut = Timeout::new(fut, Duration::from_millis(1)).compat();
                                    await!(fut)?
                                }
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
                        if orphan_messages.contains_key(&id) {
                            warn!("[Servo] Message come before waiter");
                            let msg = orphan_messages.remove(&id).unwrap();
                            if let Err(msg) = waiter.send(msg.to_result()) {
                                info!("The client for request {} is dropped, response is {:?}", id, msg);
                            }
                        } else {
                            waiters.insert(id, waiter);
                        }
                    } else {
                        warn!("[Servo] API Client dropped");
                    }
                }
            };
        }
    }
}
