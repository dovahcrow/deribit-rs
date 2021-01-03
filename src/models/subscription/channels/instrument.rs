use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum InstrumentState {
    Created,
    Started,
    Settled,
    Closed,
    Terminated,
}
#[derive(Deserialize, Serialize, Debug, Clone)]

pub struct InstrumentStateData {
    pub timestamp: bool,
    pub state: InstrumentState,
    pub instrument_name: String,
}
