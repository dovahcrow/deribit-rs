use crate::models::{SubscriptionData, SubscriptionMessage};
use futures::channel::mpsc;
use futures::Stream;
use futures::{task::Waker, Poll};
use std::pin::Pin;

pub struct DeribitSubscriptionClient {
    rx: mpsc::Receiver<SubscriptionMessage>,
}

impl DeribitSubscriptionClient {
    pub(crate) fn new(rx: mpsc::Receiver<SubscriptionMessage>) -> DeribitSubscriptionClient {
        DeribitSubscriptionClient { rx }
    }
}

impl Stream for DeribitSubscriptionClient {
    type Item = SubscriptionData;

    fn poll_next(mut self: Pin<&mut Self>, waker: &Waker) -> Poll<Option<Self::Item>> {
        let pin = Pin::new(&mut self.rx);
        match pin.poll_next(waker) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(v.params.data)),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
