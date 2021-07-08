use fehler::throw;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QuoteData {
    pub best_ask_amount: f64,
    pub best_ask_price: f64,
    pub best_bid_amount: f64,
    pub best_bid_price: f64,
    pub instrument_name: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct QuoteChannel(String);
impl<'de> Deserialize<'de> for QuoteChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        let segments: Vec<_> = s.split(".").collect();
        match segments.as_slice() {
            ["quote", instrument_name] => Ok(QuoteChannel(instrument_name.to_string())),
            _ => throw!(D::Error::invalid_value(
                Unexpected::Str(s),
                &"quote.{instrument_name}"
            )),
        }
    }
}
impl Serialize for QuoteChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::fmt::Display for QuoteChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "quote.{}", self.0)
    }
}
