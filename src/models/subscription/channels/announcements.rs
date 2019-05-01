use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AnnouncementsData {
    pub action: String,
    pub title: String,
    pub body: String,
    pub date: u64,
    pub id: u64,
    pub important: bool,
    pub number: u64,
}