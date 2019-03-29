#![feature(futures_api, async_await, await_macro)]

use crate::api_client::DeribitAPIClient;
use crate::errors::Result;
use crate::models::{JSONRPCResponse, SubscriptionMessage, WSMessage};
use crate::subscription_client::DeribitSubscriptionClient;
use futures::channel::{mpsc, oneshot};
use futures::compat::{Compat, Future01CompatExt, Sink01CompatExt, Stream01CompatExt};
use futures::{FutureExt, SinkExt, Stream, StreamExt, TryFutureExt, TryStreamExt};
use futures01::Stream as Stream01;
use log::{debug, error, info};
use serde_json::from_str;
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

#[derive(Debug)]
enum IncomingMessage {
    WSMessage(Message),
    WaiterMessage((i64, oneshot::Sender<Result<JSONRPCResponse>>)),
}

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
        waiter_rx: mpsc::Receiver<(i64, oneshot::Sender<Result<JSONRPCResponse>>)>,
        mut stx: mpsc::Sender<SubscriptionMessage>,
    ) -> Result<()> {
        let waiter_rx = waiter_rx.map(Ok).map_ok(IncomingMessage::WaiterMessage);
        let ws = ws.map_ok(IncomingMessage::WSMessage);
        let mut waiters: HashMap<i64, oneshot::Sender<Result<JSONRPCResponse>>> = HashMap::new();

        let mut stream = ws.select(waiter_rx);
        while let Some(maybe_msg) = await!(stream.next()) {
            debug!("[Servo] Message: {:?}", maybe_msg);
            match maybe_msg? {
                IncomingMessage::WSMessage(Message::Text(msg)) => {
                    let resp: WSMessage = match from_str(&msg) {
                        Ok(msg) => msg,
                        Err(e) => {
                            error!("Cannot decode rpc message {:?}", e);
                            Err(e)?
                        }
                    };

                    match resp {
                        WSMessage::Invoke(msg) => waiters.remove(&msg.id).unwrap().send(Ok(msg)).unwrap(),
                        WSMessage::Error(msg) => waiters.remove(&msg.id).unwrap().send(Err(msg.localize().into())).unwrap(),
                        WSMessage::Subscription(event) => await!(stx.send(event))?,
                    };
                }
                IncomingMessage::WSMessage(Message::Ping(_)) => {
                    println!("Received Ping");
                }
                IncomingMessage::WSMessage(Message::Pong(_)) => {
                    println!("Received Ping");
                }
                IncomingMessage::WSMessage(Message::Binary(_)) => {
                    println!("Received Binary");
                }
                IncomingMessage::WaiterMessage((id, waiter)) => {
                    waiters.insert(id, waiter);
                }
            }
        }
        Ok(())
    }
}
