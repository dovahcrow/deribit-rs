#![recursion_limit = "512"]

mod api_client;
pub mod errors;
pub mod models;
mod subscription_client;

pub use crate::api_client::{DeribitAPICallRawResult, DeribitAPICallResult, DeribitAPIClient};
pub use crate::errors::{DeribitError, Result};
pub use crate::subscription_client::{DeribitSubscriptionClient, DeribitSubscriptionLimitedClient};

use derive_builder::Builder;
use failure::Error;
use fehler::throws;
use futures::channel::{mpsc, oneshot};
use futures::{select, FutureExt, SinkExt, Stream, StreamExt, TryStreamExt};
use lazy_static::lazy_static;
use log::warn;
use log::{info, trace};
use regex::Regex;
use std::{collections::HashMap, time::Duration};
use tokio::net::TcpStream;
use tokio::time::timeout;
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
    #[builder(setter(into, strip_option), default)]
    timeout: Option<Duration>,
}

impl Deribit {
    pub fn new() -> Deribit {
        DeribitBuilder::default().build().unwrap()
    }

    pub fn builder() -> DeribitBuilder {
        DeribitBuilder::default()
    }

    #[throws(Error)]
    pub async fn connect(self) -> (DeribitAPIClient, DeribitSubscriptionClient) {
        let ws_url = if self.testnet { WS_URL_TESTNET } else { WS_URL };
        info!("Connecting");
        let (ws, _) = connect_async(Url::parse(ws_url)?).await?;

        let (wstx, wsrx) = ws.split();

        let (stx, srx) = mpsc::channel(self.subscription_buffer_size);
        let (waiter_tx, waiter_rx) = mpsc::channel(10);
        let background = Self::servo(wsrx.err_into(), waiter_rx, stx)
            .inspect(|r| {
                if let Err(e) = r {
                    warn!("[Servo] Exiting because of '{}'", e)
                }
            })
            .then(|_| async { () });

        tokio::spawn(background);

        (
            DeribitAPIClient::new(
                wstx,
                waiter_tx,
                self.timeout.unwrap_or(Duration::from_secs(3600)), // default timeout, 1H
            ),
            DeribitSubscriptionClient::new(srx),
        )
    }

    #[throws(Error)]
    async fn servo(
        ws: impl Stream<Item = Result<Message>> + Unpin,
        mut waiter_rx: mpsc::Receiver<(i64, oneshot::Sender<String>)>,
        mut stx: mpsc::Sender<String>,
    ) {
        let mut ws = ws.fuse();
        let mut waiters: HashMap<i64, oneshot::Sender<String>> = HashMap::new();

        let mut orphan_messages = HashMap::new();

        let (mut sdropped, mut cdropped) = (false, false);
        while !(sdropped && cdropped) {
            select! {
                msg = ws.next() => {
                    trace!("[Servo] Message: {:?}", msg);
                    if sdropped { continue; }
                    let msg = if let Some(msg) = msg { msg } else { Err(DeribitError::WebsocketDisconnected)? };

                    match msg? {
                        Message::Text(msg) => {
                            if let Some(cap) = RE.captures(&msg) { // TODO: If deribit returns unordered keys, then this will fail.
                                // is a API call response
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
                                    info!("[Servo] Orphan response: {:?}", msg);
                                }
                            } else {
                                // is a subscription messasge
                                let fut = stx.send(msg);
                                let fut = timeout(Duration::from_millis(1),fut, );
                                match fut.await {
                                    Ok(Ok(_)) => {}
                                    Ok(Err(ref e)) if e.is_disconnected() => sdropped = true,
                                    Ok(Err(_)) => { unreachable!("[Servo] futures::mpsc won't complain channel is full") }, // MPSC ERROR
                                    Err(_) => { warn!("[Servo] Subscription channel is full") }, // Elapsed
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
                        Message::Close(_) => {
                            trace!("[Servo] Received Close");
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
        info!("Servo exit with all receiver dropped");
        // Exit with all receiver dropped
    }
}
