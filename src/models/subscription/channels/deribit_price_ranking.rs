use fehler::throw;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeribitPriceRankingData {
    pub enabled: bool,
    pub identifier: String,
    pub original_price: f64,
    pub price: f64,
    pub timestamp: u64,
    pub weight: f64,
}

#[derive(Debug, Clone)]
pub struct DeribitPriceRankingChannel(pub String);
impl<'de> Deserialize<'de> for DeribitPriceRankingChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        let segments: Vec<_> = s.split(".").collect();
        match segments.as_slice() {
            ["deribit_price_ranking", index_name] => {
                Ok(DeribitPriceRankingChannel(index_name.to_string()))
            }
            _ => throw!(D::Error::invalid_value(
                Unexpected::Str(s),
                &"deribit_price_ranking.{index_name}"
            )),
        }
    }
}
impl Serialize for DeribitPriceRankingChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::fmt::Display for DeribitPriceRankingChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "deribit_price_ranking.{}", self.0)
    }
}
