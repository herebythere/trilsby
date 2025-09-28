use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Bookmark {
    pub id: u64,
    pub url: String,
    pub people_id: u64,
    pub deleted_at: Option<u64>,
}
