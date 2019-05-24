use crate::models::SubscriptionMessage;
use failure::Fallible;
use futures::channel::mpsc;
use futures::Stream;
use futures::{task::Context, Poll};
use pin_utils::unsafe_pinned;
use serde::de::DeserializeOwned;
use serde_json::from_str;
use std::marker::PhantomData;
use std::pin::Pin;

pub struct DeribitSubscriptionClient {
    rx: mpsc::Receiver<String>,
}

impl DeribitSubscriptionClient {
    pub(crate) fn new(rx: mpsc::Receiver<String>) -> DeribitSubscriptionClient {
        DeribitSubscriptionClient { rx }
    }

    pub fn limited<D>(self) -> DeribitSubscriptionLimitedClient<D> {
        DeribitSubscriptionLimitedClient {
            rx: self.rx,
            _ty: PhantomData,
        }
    }
}

impl Stream for DeribitSubscriptionClient {
    type Item = SubscriptionMessage;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let pin = Pin::new(&mut self.rx);
        match pin.poll_next(cx) {
            Poll::Ready(Some(v)) => {
                let data = from_str(&v).unwrap();
                Poll::Ready(Some(data))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}


pub struct DeribitSubscriptionLimitedClient<D> {
    rx: mpsc::Receiver<String>,
    _ty: PhantomData<D>,
}

impl<D> DeribitSubscriptionLimitedClient<D> {
    unsafe_pinned!(rx: mpsc::Receiver<String>);
}

impl<D: DeserializeOwned> Stream for DeribitSubscriptionLimitedClient<D> {
    type Item = Fallible<SubscriptionMessage<D>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match self.rx().poll_next(cx) {
            Poll::Ready(Some(v)) => {
                let data = from_str::<SubscriptionMessage<D>>(&v).map_err(From::from);
                Poll::Ready(Some(data))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
