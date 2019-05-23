use crate::models::SubscriptionMessage;
use futures::channel::mpsc;
use futures::Stream;
use futures::{task::Context, Poll};
use serde_json::from_str;
use std::pin::Pin;

pub struct DeribitSubscriptionClient {
    rx: mpsc::Receiver<String>,
}

impl DeribitSubscriptionClient {
    pub(crate) fn new(rx: mpsc::Receiver<String>) -> DeribitSubscriptionClient {
        DeribitSubscriptionClient { rx }
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
