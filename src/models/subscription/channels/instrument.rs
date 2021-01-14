use fehler::throw;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

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

#[derive(Debug, Clone)]
pub struct InstrumentStateChannel(String, String);
impl<'de> Deserialize<'de> for InstrumentStateChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        let segments: Vec<_> = s.split(".").collect();
        match segments.as_slice() {
            ["instrument", "state", kind, currency] => Ok(InstrumentStateChannel(
                kind.to_string(),
                currency.to_string(),
            )),
            _ => throw!(D::Error::invalid_value(
                Unexpected::Str(s),
                &"instrument.state.{kind}.{currency}"
            )),
        }
    }
}
impl Serialize for InstrumentStateChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("instrument.state.{}.{}", self.0, self.1))
    }
}
