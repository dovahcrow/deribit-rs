use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum InstrumentState {
    Created,
    Started,
    Settled,
    Closed,
    Terminated,
}
#[derive(Deserialize, Serialize, Debug, Clone)]

pub struct InstrumentStateData {
    pub timestamp: u64,
    pub state: InstrumentState,
    pub instrument_name: String,
}
