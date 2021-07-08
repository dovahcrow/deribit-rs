use fehler::throw;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MarkPriceOptionData {
    pub synthetic_future: Option<SyntheticFuture>,
    pub instrument_name: String,
    pub iv: f64,
    pub mark_price: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]

pub struct SyntheticFuture {
    pub mark_price: f64,
    pub instrument_name: String,
}

#[derive(Debug, Clone)]
pub struct MarkPriceOptionChannel(String);
impl<'de> Deserialize<'de> for MarkPriceOptionChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        let segments: Vec<_> = s.split(".").collect();
        match segments.as_slice() {
            ["markprice", "options", index_name] => {
                Ok(MarkPriceOptionChannel(index_name.to_string()))
            }
            _ => throw!(D::Error::invalid_value(
                Unexpected::Str(s),
                &"markprice.options.{index_name}"
            )),
        }
    }
}
impl Serialize for MarkPriceOptionChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::fmt::Display for MarkPriceOptionChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "markprice.options.{}", self.0)
    }
}
