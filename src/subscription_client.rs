use futures::future::ok;
use futures::sink::Sink;
use futures::stream::{SplitSink, Stream};
use futures::channel::{mpsc, oneshot};
use futures::Future;
use std::collections::HashMap;

use serde_json::from_str;

use failure::Error;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::Message;
use url::Url;

use crate::models::{JSONRPCErrorResponse, JSONRPCInvokeResponse, JSONRPCResponse, JSONRPCSubscriptionResponse};

use log::{error, info};

pub struct DeribitSubscriptionClient {
    rx: mpsc::Receiver<JSONRPCResponse>,
}

impl DeribitSubscriptionClient {
    pub(crate) fn new(rx: mpsc::Receiver<JSONRPCResponse>) -> DeribitSubscriptionClient {
        DeribitSubscriptionClient { rx }
    }
}
