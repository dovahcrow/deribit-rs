#![feature(async_await, specialization)]
#![recursion_limit = "512"]

mod api_client;
pub mod errors;
pub mod models;
mod subscription_client;

pub use crate::api_client::{DeribitAPICallRawResult, DeribitAPICallResult, DeribitAPIClient};
pub use crate::subscription_client::{DeribitSubscriptionClient, DeribitSubscriptionLimitedClient};

use crate::errors::DeribitError;
use derive_builder::Builder;
use failure::Fallible;
use futures::channel::{mpsc, oneshot};
use futures::compat::{Future01CompatExt, Sink01CompatExt, Stream01CompatExt};
use futures::{select, FutureExt, SinkExt, Stream, StreamExt, TryFutureExt, TryStreamExt};
use futures01::Stream as Stream01;
use lazy_static::lazy_static;
use log::warn;
use log::{info, trace};
use regex::Regex;
use std::collections::HashMap;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::timer::Timeout;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::Message;
use url::Url;


lazy_static! {
    static ref RE: Regex = Regex::new(r#""jsonrpc":"2.0","id":(\d+),"#).unwrap();
}

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
        let (ws, _) = connect_async(Url::parse(ws_url)?).compat().await?;

        let (wstx, wsrx) = ws.split();
        let (stx, srx) = mpsc::channel(self.subscription_buffer_size);
        let (waiter_tx, waiter_rx) = mpsc::channel(10);
        let background = Self::servo(wsrx.compat().err_into(), waiter_rx, stx).map_err(|e| {
            warn!("[Servo] Exiting because of '{}'", e);
        });

        tokio::spawn(background.boxed().compat());

        Ok((
            DeribitAPIClient::new(wstx.sink_compat(), waiter_tx),
            DeribitSubscriptionClient::new(srx),
        ))
    }

    async fn servo(
        ws: impl Stream<Item = Fallible<Message>> + Unpin,
        mut waiter_rx: mpsc::Receiver<(i64, oneshot::Sender<String>)>,
        mut stx: mpsc::Sender<String>,
    ) -> Fallible<()> {
        let mut ws = ws.fuse();
        let mut waiters: HashMap<i64, oneshot::Sender<String>> = HashMap::new();

        let mut orphan_messages = HashMap::new();

        let (mut sdropped, mut cdropped) = (false, false);
        while !sdropped && !cdropped {
            select! {
                msg = ws.next() => {
                    trace!("[Servo] Message: {:?}", msg);
                    if sdropped { continue; }
                    let msg = if let Some(msg) = msg { msg } else { Err(DeribitError::WebsocketDisconnected)? };

                    match msg? {
                        Message::Text(msg) => {
                            if let Some(cap) = RE.captures(&msg) {
                                let id_str = cap.get(1).expect("No captured group in a capture result, this cannot happen").as_str();
                                let id = id_str.parse().expect("Cannot parse integer while it is deemed as integer by regex, this cannot happen");
                                let waiter = match waiters.remove(&id) {
                                    Some(waiter) => waiter,
                                    None => {
                                        orphan_messages.insert(id, msg);
                                        continue;
                                    }
                                };

                                if let Err(msg) = waiter.send(msg) {
                                    info!("[Servo] The client for request {} is dropped, response is {:?}", id, msg);
                                }
                            } else {
                                let fut = stx.send(msg).compat();
                                let fut = Timeout::new(fut, Duration::from_millis(1)).compat();
                                match fut.await.map_err(|e| e.into_inner()) {
                                    Ok(_) => {}
                                    Err(Some(ref e)) if e.is_disconnected() => sdropped = true,
                                    Err(Some(e)) => { unreachable!("[Servo] futures::mpsc won't complain channel is full") },
                                    Err(None) => { warn!("[Servo] Subscription channel is full") }
                                }
                            }
                        }
                        Message::Ping(_) => {
                            trace!("[Servo] Received Ping");
                        }
                        Message::Pong(_) => {
                            trace!("[Servo] Received Ping");
                        }
                        Message::Binary(_) => {
                            trace!("[Servo] Received Binary");
                        }
                    }
                }
                waiter = waiter_rx.next() => {
                    if let Some((id, waiter)) = waiter {
                        if orphan_messages.contains_key(&id) {
                            info!("[Servo] Message come before waiter");
                            let msg = orphan_messages.remove(&id).unwrap();
                            if let Err(msg) = waiter.send(msg) {
                                info!("[Servo] The client for request {} is dropped, response is {:?}", id, msg);
                            }
                        } else {
                            waiters.insert(id, waiter);
                        }
                    } else {
                        cdropped = true;
                        info!("[Servo] API Client dropped");
                    }
                }
            };
        }

        Ok(()) // Exit with all receiver dropped
    }
}
