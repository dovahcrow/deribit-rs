use crate::errors::DeribitError;
use crate::models::{JSONRPCRequest, JSONRPCResponse, Request};
use crate::WSStream;
use failure::Fallible;
use futures::channel::{mpsc, oneshot};
use futures::compat::Compat01As03Sink;
use futures::task::Context;
use futures::{Future, Poll, SinkExt};
use futures01::stream::SplitSink as SplitSink01;
use log::trace;
use pin_utils::unsafe_pinned;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_str, to_string};
use std::marker::PhantomData;
use std::pin::Pin;
use tungstenite::Message;

type SplitWSCompatStream = Compat01As03Sink<SplitSink01<WSStream>, Message>;

pub struct DeribitAPIClient {
    wstx: SplitWSCompatStream,
    waiter_tx: mpsc::Sender<(i64, oneshot::Sender<String>)>,
    id: i64,
}

impl DeribitAPIClient {
    pub(crate) fn new(
        wstx: SplitWSCompatStream,
        waiter_tx: mpsc::Sender<(i64, oneshot::Sender<String>)>,
    ) -> DeribitAPIClient {
        DeribitAPIClient {
            wstx: wstx,
            waiter_tx: waiter_tx,

            id: 0,
        }
    }

    pub async fn call_raw<'a, R>(
        &'a mut self,
        request: R,
    ) -> Fallible<DeribitAPICallRawResult<R::Response>>
    where
        R: Request + Serialize + 'a,
    {
        let (waiter_tx, waiter_rx) = oneshot::channel();
        let req = JSONRPCRequest {
            id: self.id,
            method: R::METHOD.into(),
            params: request,
        };
        self.id += 1;

        let payload = to_string(&req)?;
        trace!("[Deribit] Request: {}", payload);
        self.wstx.send(Message::Text(payload)).await?;
        self.waiter_tx.send((req.id, waiter_tx)).await?;
        Ok(DeribitAPICallRawResult::new(waiter_rx))
    }

    pub async fn call<'a, R>(
        &'a mut self,
        request: R,
    ) -> Fallible<DeribitAPICallResult<R::Response>>
    where
        R: Request + Serialize + 'a,
    {
        let resp: DeribitAPICallRawResult<R::Response> = self.call_raw(request).await?;
        Ok(DeribitAPICallResult::new(resp))
    }
}

pub struct DeribitAPICallRawResult<R> {
    rx: oneshot::Receiver<String>,
    _ty: PhantomData<R>,
}

impl<R> DeribitAPICallRawResult<R> {
    pub(crate) fn new(rx: oneshot::Receiver<String>) -> Self {
        DeribitAPICallRawResult {
            rx: rx,
            _ty: PhantomData,
        }
    }
    unsafe_pinned!(rx: oneshot::Receiver<String>);
}

impl<R> Future for DeribitAPICallRawResult<R>
where
    R: DeserializeOwned,
{
    type Output = Fallible<JSONRPCResponse<R>>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Fallible<JSONRPCResponse<R>>> {
        self.rx().poll(cx).map(|result| {
            let resp = result?;
            let result: JSONRPCResponse<R> = from_str(&resp)?;
            Ok(result)
        })
    }
}

pub struct DeribitAPICallResult<R> {
    inner: DeribitAPICallRawResult<R>,
}

impl<R> DeribitAPICallResult<R> {
    pub(crate) fn new(inner: DeribitAPICallRawResult<R>) -> Self {
        DeribitAPICallResult { inner: inner }
    }
    unsafe_pinned!(inner: DeribitAPICallRawResult<R>);
}

impl<R> Future for DeribitAPICallResult<R>
where
    R: DeserializeOwned,
{
    type Output = Fallible<R>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Fallible<R>> {
        match self.inner().poll(cx) {
            Poll::Ready(Ok(resp)) => Poll::Ready(resp.result.left_result().map_err(|e| {
                DeribitError::RemoteError {
                    code: e.code,
                    message: e.message,
                }
                .into()
            })),
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }
}
