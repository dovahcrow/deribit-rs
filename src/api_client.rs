use crate::errors::{DeribitError, Result};
use crate::models::{JSONRPCRequest, JSONRPCResponse, Request};
use crate::WSStream;
use failure::Error;
use fehler::throws;
use futures::{
    channel::{mpsc, oneshot},
    stream::SplitSink,
    task::{Context, Poll},
    {Future, SinkExt},
};
use log::{error, trace};
use pin_project::pin_project;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};
use std::{
    convert::Into, marker::PhantomData, pin::Pin, result::Result as StdResult, time::Duration,
};
use tokio::time::{error::Elapsed, timeout, Timeout};
use tungstenite::Message;

pub struct DeribitAPIClient {
    wstx: SplitSink<WSStream, Message>,
    waiter_tx: mpsc::Sender<(i64, oneshot::Sender<String>)>,
    timeout: Duration,
    id: i64,
}

impl DeribitAPIClient {
    pub(crate) fn new(
        wstx: SplitSink<WSStream, Message>,
        waiter_tx: mpsc::Sender<(i64, oneshot::Sender<String>)>,
        timeout: Duration,
    ) -> DeribitAPIClient {
        DeribitAPIClient {
            wstx: wstx,
            waiter_tx: waiter_tx,
            timeout: timeout,
            id: 0,
        }
    }

    #[throws(Error)]
    pub async fn call_raw<'a, R>(&'a mut self, request: R) -> DeribitAPICallRawResult<R::Response>
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
        trace!("[API Client] Request: {}", payload);
        self.wstx.send(Message::Text(payload)).await?;
        self.waiter_tx.send((req.id, waiter_tx)).await?;
        DeribitAPICallRawResult::new(waiter_rx, self.timeout)
    }

    #[throws(Error)]
    pub async fn call<'a, R>(&'a mut self, request: R) -> DeribitAPICallResult<R::Response>
    where
        R: Request + Serialize + 'a,
    {
        let resp: DeribitAPICallRawResult<R::Response> = self.call_raw(request).await?;
        DeribitAPICallResult::new(resp)
    }
}

#[pin_project]
pub struct DeribitAPICallRawResult<R> {
    #[pin]
    rx: Timeout<oneshot::Receiver<String>>,
    _ty: PhantomData<R>,
}

impl<R> DeribitAPICallRawResult<R> {
    pub(crate) fn new(rx: oneshot::Receiver<String>, expiry: Duration) -> Self {
        DeribitAPICallRawResult {
            rx: timeout(expiry, rx),
            _ty: PhantomData,
        }
    }
}

impl<R> Future for DeribitAPICallRawResult<R>
where
    R: DeserializeOwned,
{
    type Output = Result<JSONRPCResponse<R>>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<JSONRPCResponse<R>>> {
        let this = self.project();
        match this.rx.poll(cx) {
            Poll::Ready(Ok(ret)) => Poll::Ready(match ret {
                Ok(resp) => {
                    let result: StdResult<JSONRPCResponse<R>, _> = from_str(&resp);
                    if let Err(_) = result.as_ref() {
                        error!("[API Client] Cannot deserialize RPC response: {}", resp);
                    }
                    result.map_err(Into::into)
                }
                Err(err) => Err(err.into()),
            }),
            Poll::Ready(Err(Elapsed { .. })) => {
                Poll::Ready(Err(DeribitError::RequestTimeout.into()))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
#[pin_project]
pub struct DeribitAPICallResult<R> {
    #[pin]
    inner: DeribitAPICallRawResult<R>,
}

impl<R> DeribitAPICallResult<R> {
    pub(crate) fn new(inner: DeribitAPICallRawResult<R>) -> Self {
        DeribitAPICallResult { inner: inner }
    }
}

impl<R> Future for DeribitAPICallResult<R>
where
    R: DeserializeOwned,
{
    type Output = Result<R>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<R>> {
        let this = self.project();
        match this.inner.poll(cx) {
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
