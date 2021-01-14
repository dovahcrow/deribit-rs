use fehler::throw;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AnnouncementsData {
    pub action: String,
    pub title: String,
    pub body: String,
    pub date: u64,
    pub id: u64,
    pub important: bool,
    pub number: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct AnnouncementsChannel;
impl<'de> Deserialize<'de> for AnnouncementsChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        if s == "announcements" {
            Ok(AnnouncementsChannel)
        } else {
            throw!(D::Error::invalid_value(
                Unexpected::Str(&s),
                &"announcements"
            ))
        }
    }
}
impl Serialize for AnnouncementsChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("announcements")
    }
}
