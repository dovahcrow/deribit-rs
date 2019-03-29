mod authentication;
mod subscription;
mod support;

use crate::errors::Result;
use crate::models::{JSONRPCRequest};
use crate::WSStream;
use futures::channel::{mpsc, oneshot};
use futures::compat::Compat01As03Sink;
use futures::SinkExt;
use futures01::stream::SplitSink as SplitSink01;
use log::debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_value, to_string, Value};
use tungstenite::Message;

type SplitWSCompatStream = Compat01As03Sink<SplitSink01<WSStream>, Message>;

pub struct DeribitAPIClient {
    wstx: SplitWSCompatStream,
    waiter_tx: mpsc::Sender<(i64, oneshot::Sender<Result<Value>>)>,

    id: i64,
}

impl DeribitAPIClient {
    pub(crate) fn new(wstx: SplitWSCompatStream, waiter_tx: mpsc::Sender<(i64, oneshot::Sender<Result<Value>>)>) -> DeribitAPIClient {
        DeribitAPIClient {
            wstx: wstx,
            waiter_tx: waiter_tx,

            id: 0,
        }
    }

    pub async fn request<'a, R, Q>(&'a mut self, method: &'a str, params: Option<Q>) -> Result<R>
    where
        R: DeserializeOwned,
        Q: Serialize + 'a,
    {
        let (waiter_tx, waiter_rx) = oneshot::channel();
        let req = JSONRPCRequest {
            id: self.id,
            method: method.into(),
            params: params,
        };
        self.id += 1;

        let payload = to_string(&req)?;
        debug!("[Deribit] Request: {}", payload);
        await!(self.wstx.send(Message::Text(payload)))?;
        await!(self.waiter_tx.send((req.id, waiter_tx)))?;

        let resp = await!(waiter_rx)??;
        debug!("[Deribit] Response: {:?}", resp);
        Ok(from_value(resp)?)
    }
}
