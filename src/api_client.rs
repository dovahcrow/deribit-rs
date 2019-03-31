mod account;
mod authentication;
mod session_management;
mod subscription;
mod support;
mod trading;

use crate::errors::{DeribitError, Result};
use crate::models::{JSONRPCRequest, JSONRPCResponse};
use crate::WSStream;
use futures::channel::{mpsc, oneshot};
use futures::compat::Compat01As03Sink;
use futures::SinkExt;
use futures01::stream::SplitSink as SplitSink01;
use log::debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_value, to_string};
use tungstenite::Message;

// pub struct DeribitAPICallResult<R> {
//     rx: oneshot::Receiver<Result<JSONRPCResponse>>,
//     _marker: PhantomData<R>
// }
// impl<R> DeribitAPICallResult<R> {
//     pub fn new(rx: oneshot::Receiver<Result<JSONRPCResponse>>) -> Self {
//         DeribitAPICallResult{
//             rx: rx,
//             _marker: PhantomData
//         }
//     }
// }

// impl<R> Future for DeribitAPICallResult<R> where R: DeserializeOwned {
//     type Output = Result<R>;
//     fn poll(self: Pin<&mut Self>, waker: &Waker) -> Poll<Self::Output> {
//         match self.rx.poll(waker) {
//             Poll::Ready(resp) => {
//                 resp.and_then(|v| {
//                     v.map(|resp| {
//                         let result = from_value(resp.result.unwrap()).unwrap();
//                         JSONRPCResponse {
//                                     jsonrpc: resp.jsonrpc,
//                                     id: resp.id,
//                                     testnet: resp.testnet,
//                                     error: None,
//                                     result: Some(result),
//                                     us_in: resp.us_in,
//                                     us_out: resp.us_out,
//                                     us_diff: resp.us_diff,
//                                 }
//                     })
//                 })
//             }
//             Poll::Pending => Poll::Pending
//         }
//     }
// }

type SplitWSCompatStream = Compat01As03Sink<SplitSink01<WSStream>, Message>;

pub struct DeribitAPIClient {
    wstx: SplitWSCompatStream,
    waiter_tx: mpsc::Sender<(i64, oneshot::Sender<Result<JSONRPCResponse>>)>,

    id: i64,
}

impl DeribitAPIClient {
    pub(crate) fn new(
        wstx: SplitWSCompatStream,
        waiter_tx: mpsc::Sender<(i64, oneshot::Sender<Result<JSONRPCResponse>>)>,
    ) -> DeribitAPIClient {
        DeribitAPIClient {
            wstx: wstx,
            waiter_tx: waiter_tx,

            id: 0,
        }
    }

    pub async fn request_raw<'a, Q, R>(
        &'a mut self,
        method: &'a str,
        params: Option<Q>,
    ) -> Result<JSONRPCResponse<R>>
    where
        Q: Serialize + 'a,
        R: DeserializeOwned,
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
        let resp = await!(waiter_rx).map_err(|_| DeribitError::ServoExited)??;
        let result = from_value(resp.result.unwrap()).unwrap();
        let resp = JSONRPCResponse {
            jsonrpc: resp.jsonrpc,
            id: resp.id,
            testnet: resp.testnet,
            error: None,
            result: Some(result),
            us_in: resp.us_in,
            us_out: resp.us_out,
            us_diff: resp.us_diff,
        };
        Ok(resp)
    }

    pub async fn request<'a, R, Q>(&'a mut self, method: &'a str, params: Option<Q>) -> Result<R>
    where
        R: DeserializeOwned,
        Q: Serialize + 'a,
    {
        let resp = await!(self.request_raw(method, params))?;
        Ok(resp.result.unwrap())
    }
}
