use fehler::throw;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PerpetualData {
    pub interest: f64,
}

#[derive(Debug, Clone)]
pub struct PerpetualChannel(String, String);
impl<'de> Deserialize<'de> for PerpetualChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        let segments: Vec<_> = s.split(".").collect();
        match segments.as_slice() {
            ["perpetual", instrument_name, interval] => Ok(PerpetualChannel(
                instrument_name.to_string(),
                interval.to_string(),
            )),
            _ => throw!(D::Error::invalid_value(
                Unexpected::Str(s),
                &"perpetual.{instrument_name}.{interval}"
            )),
        }
    }
}
impl Serialize for PerpetualChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::fmt::Display for PerpetualChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "perpetual.{}.{}", self.0, self.1)
    }
}
