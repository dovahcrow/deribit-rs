use crate::errors::Result;
use crate::models::SubscriptionMessage;
use futures::channel::mpsc;
use futures::task::{Context, Poll};
use futures::Stream;
use log::warn;
use pin_project::pin_project;
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
    type Item = Result<SubscriptionMessage>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let pin = Pin::new(&mut self.rx);
        match pin.poll_next(cx) {
            Poll::Ready(Some(v)) => {
                let data = from_str::<SubscriptionMessage>(&v).map_err(From::from);
                if let Err(_) = data.as_ref() {
                    warn!(
                        "[Subscription Client] Cannot deserialize subscription message: {}",
                        v
                    );
                }
                Poll::Ready(Some(data))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[pin_project]
pub struct DeribitSubscriptionLimitedClient<D> {
    #[pin]
    rx: mpsc::Receiver<String>,
    _ty: PhantomData<D>,
}

impl<D: DeserializeOwned> Stream for DeribitSubscriptionLimitedClient<D> {
    type Item = Result<SubscriptionMessage<D>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let this = self.project();
        match this.rx.poll_next(cx) {
            Poll::Ready(Some(v)) => {
                let data = from_str::<SubscriptionMessage<D>>(&v).map_err(From::from);
                if let Err(_) = data.as_ref() {
                    warn!(
                        "[Subscription Client] Cannot deserialize subscription message: {}",
                        v
                    );
                }
                Poll::Ready(Some(data))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
