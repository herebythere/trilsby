use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: u64,
    pub people_id: u64,
    pub title: String,
    pub deleted_at: Option<u64>,
}
