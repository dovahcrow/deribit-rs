use failure::Fail;

#[derive(Fail, Debug)]
pub enum DeribitError {
    #[fail(display = "Dummy error")]
    Dummy,
    #[fail(
        display = "Deribit remote error {{code: {}, message: {}}}",
        code, message
    )]
    RemoteError { code: i64, message: String },
    #[fail(display = "The background servo pulling message exited")]
    ServoExited,
    #[fail(display = "Unknown currency {}", _0)]
    UnknownCurrency(String),
    #[fail(display = "Websocket disconnected")]
    WebsocketDisconnected,
}
