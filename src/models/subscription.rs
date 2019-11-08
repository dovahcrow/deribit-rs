mod channels;

use crate::models::Request;
use serde::{Deserialize, Serialize};

pub use channels::AnnouncementsData;
pub use channels::DeribitPriceIndexData;
pub use channels::DeribitPriceRankingData;
pub use channels::EstimatedExpirationPriceData;
pub use channels::MarkPriceOptionData;
pub use channels::PerpetualData;
pub use channels::QuoteData;
pub use channels::TradesData;
pub use channels::UserOrdersData;
pub use channels::UserPortfolioData;
pub use channels::UserTradesData;
pub use channels::{BookData, Delta, GroupedBookData, OrderBookDelta};
pub use channels::{Greeks, Stats, TickerData};

#[derive(Deserialize, Serialize, Clone, Debug)]
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

#[derive(Deserialize, Serialize, Clone, Debug)]
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SubscribeResponse(pub Vec<String>);

impl Request for PublicSubscribeRequest {
    const METHOD: &'static str = "public/subscribe";
    type Response = SubscribeResponse;
}

impl Request for PrivateSubscribeRequest {
    const METHOD: &'static str = "private/subscribe";
    type Response = SubscribeResponse;
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PublicUnsubscribeRequest {
    pub channels: Vec<String>,
}

impl PublicUnsubscribeRequest {
    pub fn new(channels: &[String]) -> Self {
        Self {
            channels: channels.to_vec(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PrivateUnsubscribeRequest {
    pub channels: Vec<String>,
}

impl PrivateUnsubscribeRequest {
    pub fn new(channels: &[String]) -> Self {
        Self {
            channels: channels.to_vec(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UnsubscribeResponse(pub Vec<String>);

impl Request for PublicUnsubscribeRequest {
    const METHOD: &'static str = "public/unsubscribe";
    type Response = UnsubscribeResponse;
}

impl Request for PrivateUnsubscribeRequest {
    const METHOD: &'static str = "private/unsubscribe";
    type Response = UnsubscribeResponse;
}
