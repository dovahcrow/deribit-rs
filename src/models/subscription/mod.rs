mod channels;

use crate::models::jsonrpc::JSONRPCVersion;
use crate::models::Request;
pub use channels::{AnnouncementsChannel, AnnouncementsData};
pub use channels::{
    BookChannel, BookData, Delta, GroupedBookChannel, GroupedBookData, OrderBookDelta,
};
pub use channels::{DeribitPriceIndexChannel, DeribitPriceIndexData};
pub use channels::{DeribitPriceRankingChannel, DeribitPriceRankingData};
pub use channels::{EstimatedExpirationPriceChannel, EstimatedExpirationPriceData};
pub use channels::{Greeks, Stats, TickerChannel, TickerData};
pub use channels::{InstrumentState, InstrumentStateChannel, InstrumentStateData};
pub use channels::{MarkPriceOptionChannel, MarkPriceOptionData};
pub use channels::{PerpetualChannel, PerpetualData};
pub use channels::{QuoteChannel, QuoteData};
pub use channels::{TradesChannel, TradesData};
pub use channels::{UserChangesChannel, UserChangesData};
pub use channels::{UserOrdersChannel, UserOrdersData};
pub use channels::{UserPortfolioChannel, UserPortfolioData};
pub use channels::{UserTradesChannel, UserTradesData};
use serde::{Deserialize, Serialize};

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

// Subscription messages

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SubscriptionMessage<D = SubscriptionData> {
    pub jsonrpc: JSONRPCVersion,
    pub method: SubscriptionMethod,
    pub params: SubscriptionParams<D>,
}

impl SubscriptionMessage {
    pub fn is_subscription(&self) -> bool {
        self.method.is_subscription()
    }
    pub fn is_heartbeat(&self) -> bool {
        self.method.is_heartbeat()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Copy)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionMethod {
    Subscription,
    Heartbeat,
}

impl SubscriptionMethod {
    pub fn is_subscription(self) -> bool {
        match self {
            SubscriptionMethod::Subscription => true,
            SubscriptionMethod::Heartbeat => false,
        }
    }
    pub fn is_heartbeat(&self) -> bool {
        match self {
            SubscriptionMethod::Subscription => false,
            SubscriptionMethod::Heartbeat => true,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SubscriptionParams<D = SubscriptionData> {
    Subscription(D),
    Heartbeat { r#type: HeartbeatType },
}

impl SubscriptionParams {
    pub fn is_subscription(&self) -> bool {
        match self {
            SubscriptionParams::Subscription { .. } => true,
            SubscriptionParams::Heartbeat { .. } => false,
        }
    }
    pub fn is_heartbeat(&self) -> bool {
        match self {
            SubscriptionParams::Subscription { .. } => false,
            SubscriptionParams::Heartbeat { .. } => true,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum HeartbeatType {
    Heartbeat,
    TestRequest,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WithChannel<C, D> {
    pub channel: C,
    pub data: D,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum SubscriptionData {
    Announcements(WithChannel<AnnouncementsChannel, AnnouncementsData>),
    Book(WithChannel<BookChannel, BookData>),
    DeribitPriceIndex(WithChannel<DeribitPriceIndexChannel, DeribitPriceIndexData>),
    DeribitPriceRanking(WithChannel<DeribitPriceRankingChannel, Vec<DeribitPriceRankingData>>),
    EstimatedExpirationPrice(
        WithChannel<EstimatedExpirationPriceChannel, EstimatedExpirationPriceData>,
    ),
    GroupedBook(WithChannel<GroupedBookChannel, GroupedBookData>),
    InstrumentState(WithChannel<InstrumentStateChannel, InstrumentStateData>),
    MarkPriceOption(WithChannel<MarkPriceOptionChannel, Vec<MarkPriceOptionData>>),
    Perpetual(WithChannel<PerpetualChannel, PerpetualData>),
    Quote(WithChannel<QuoteChannel, QuoteData>),
    Ticker(WithChannel<TickerChannel, TickerData>),
    Trades(WithChannel<TradesChannel, Vec<TradesData>>), // This should be put after user trades otherwise all usertrades will be deserialized to trades
    UserOrders(WithChannel<UserOrdersChannel, UserOrdersData>),
    UserOrdersBatch(WithChannel<UserOrdersChannel, Vec<UserOrdersData>>),
    UserPortfolio(WithChannel<UserPortfolioChannel, UserPortfolioData>),
    UserTrades(WithChannel<UserTradesChannel, Vec<UserTradesData>>),
}
