use crate::models::{Either, HeartbeatMessage};
use crate::models::{SubscriptionData, SubscriptionMessage};
use futures::channel::mpsc;
use futures::Stream;
use futures::{task::Context, Poll};
use std::pin::Pin;

pub struct DeribitSubscriptionClient {
    rx: mpsc::Receiver<Either<SubscriptionMessage, HeartbeatMessage>>,
}

impl DeribitSubscriptionClient {
    pub(crate) fn new(
        rx: mpsc::Receiver<Either<SubscriptionMessage, HeartbeatMessage>>,
    ) -> DeribitSubscriptionClient {
        DeribitSubscriptionClient { rx }
    }
}

impl Stream for DeribitSubscriptionClient {
    type Item = Either<SubscriptionData, HeartbeatMessage>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let pin = Pin::new(&mut self.rx);
        match pin.poll_next(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(v.map_left(|v| v.params.data))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
