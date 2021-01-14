use fehler::throw;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Delta {
    New,
    Change,
    Delete,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OrderBookDelta(pub Delta, pub f64, pub f64);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BookData {
    pub asks: Vec<OrderBookDelta>,
    pub bids: Vec<OrderBookDelta>,
    pub change_id: i64,
    pub instrument_name: String,
    pub prev_change_id: Option<i64>,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct BookChannel(String, String);
impl<'de> Deserialize<'de> for BookChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        let segments: Vec<_> = s.split(".").collect();
        match segments.as_slice() {
            ["book", instrument_name, interval] => Ok(BookChannel(
                instrument_name.to_string(),
                interval.to_string(),
            )),
            _ => throw!(D::Error::invalid_value(
                Unexpected::Str(s),
                &"book.{instrument_name}.{interval}"
            )),
        }
    }
}
impl Serialize for BookChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("book.{}.{}", self.0, self.1))
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GroupedBookData {
    pub asks: Vec<(f64, f64)>,
    pub bids: Vec<(f64, f64)>,
    pub change_id: i64,
    pub instrument_name: String,
    pub prev_change_id: Option<i64>,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct GroupedBookChannel(String, String, String, String);
impl<'de> Deserialize<'de> for GroupedBookChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        let segments: Vec<_> = s.split(".").collect();
        match segments.as_slice() {
            ["book", instrument_name, group, depth, interval] => Ok(GroupedBookChannel(
                instrument_name.to_string(),
                group.to_string(),
                depth.to_string(),
                interval.to_string(),
            )),
            _ => throw!(D::Error::invalid_value(
                Unexpected::Str(s),
                &"book.{instrument_name}.{group}.{depth}.{interval}"
            )),
        }
    }
}
impl Serialize for GroupedBookChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("book.{}.{}.{}.{}", self.0, self.1, self.2, self.3))
    }
}
