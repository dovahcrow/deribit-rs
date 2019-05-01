mod channels;

use crate::models::Request;
use serde_derive::{Deserialize, Serialize};

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
