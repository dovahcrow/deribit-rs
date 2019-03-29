mod authentication;
mod message;
mod subscription;
mod support;

pub use authentication::{AuthRequest, AuthResponse, GrantType};
pub(crate) use message::{JSONRPCError, JSONRPCRequest, JSONRPCResponse, SubscriptionData, SubscriptionMessage, WSMessage};
use serde_derive::{Deserialize, Serialize};
pub use subscription::channel::{BookInstrumentNameIntervalRequest, BookInstrumentNameIntervalResponse, OrderBookDelta};
pub use subscription::channel::{UserPortfolioCurrencyRequest, UserPortfolioCurrencyResponse};
pub use subscription::{SubscribeRequest, SubscribeResponse};
pub use support::{GetTimeResponse, HelloRequest, HelloResponse, TestRequest, TestResponse};

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum Currency {
    BTC,
    ETH,
}
