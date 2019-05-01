mod channel;

use crate::models::Request;
use serde_derive::{Deserialize, Serialize};

pub use channel::TradesData;
pub use channel::UserOrdersData;
pub use channel::UserPortfolioData;
pub use channel::UserTradesData;
pub use channel::{BookData, Delta, GroupedBookData, OrderBookDelta};
pub use channel::{Greeks, Stats, TickerData};

#[derive(Serialize, Debug, Clone)]
pub struct PublicSubscribeRequest {
    pub channels: Vec<String>,
}

impl PublicSubscribeRequest {
    pub fn new(channels: &[String]) -> Self {
        Self {
            channels: channels.to_vec(),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct PrivateSubscribeRequest {
    pub channels: Vec<String>,
}

impl PrivateSubscribeRequest {
    pub fn new(channels: &[String]) -> Self {
        Self {
            channels: channels.to_vec(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct SubscribeResponse(pub Vec<String>);

impl Request for PublicSubscribeRequest {
    const METHOD: &'static str = "public/subscribe";
    type Response = SubscribeResponse;
}

impl Request for PrivateSubscribeRequest {
    const METHOD: &'static str = "private/subscribe";
    type Response = SubscribeResponse;
}
