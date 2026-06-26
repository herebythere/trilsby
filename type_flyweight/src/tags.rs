use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: u64,
    pub people_id: u64, // not required, internal not contributed
    pub title: String,
    pub deleted_at: Option<u64>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct TagToMark {
    pub id: u64,
    pub tag_id: u64,
    pub marker_id: u64,
    pub deleted_at: Option<u64>,
}
