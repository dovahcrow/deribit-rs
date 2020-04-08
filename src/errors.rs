use futures::channel::mpsc::SendError;
use futures::channel::oneshot::Canceled;
use serde_json::Error as JsonError;
use std::result::Result as StdResult;
use thiserror::Error;
use tungstenite::error::Error as WebsocketError;
use url::ParseError;

pub type Result<R> = StdResult<R, DeribitError>;

#[derive(Error, Debug)]
pub enum DeribitError {
    #[error("Dummy error")]
    Dummy,
    #[error("Deribit remote error {{code: {code}, message: {message}}}")]
    RemoteError { code: i64, message: String },
    #[error("The background servo pulling message exited")]
    ServoExited,
    #[error("Unknown currency {0}")]
    UnknownCurrency(String),
    #[error("Unknown asset kind {0}")]
    UnknownAssetKind(String),
    #[error("Websocket disconnected")]
    WebsocketDisconnected,

    #[error("oneshot channel canceled on the other side: {0}")]
    CanceledError(#[from] Canceled),
    #[error("cannot parse url: {0}")]
    ParseError(#[from] ParseError),
    #[error("underlying websocket reported an error: {0}")]
    WebsocketError(#[from] WebsocketError),
    #[error("cannot send message to channel: {0}")]
    SendError(#[from] SendError),
    #[error("JSON serialization error: {0}")]
    JsonError(#[from] JsonError),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}
