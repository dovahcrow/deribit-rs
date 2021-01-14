use fehler::throw;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeribitPriceIndexData {
    pub index_name: String,
    pub price: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct DeribitPriceIndexChannel(String);
impl<'de> Deserialize<'de> for DeribitPriceIndexChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        let segments: Vec<_> = s.split(".").collect();
        match segments.as_slice() {
            ["deribit_price_index", index_name] => {
                Ok(DeribitPriceIndexChannel(index_name.to_string()))
            }
            _ => throw!(D::Error::invalid_value(
                Unexpected::Str(s),
                &"deribit_price_index.{index_name}"
            )),
        }
    }
}
impl Serialize for DeribitPriceIndexChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("deribit_price_index.{}", self.0))
    }
}
