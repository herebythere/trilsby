use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TagKind {
    pub id: u64,
    pub kind: String,
    pub deleted_at: Option<u64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: u64,
    pub people_id: u64,
    pub tag_kind_id: u64,
    pub bookmark_id: u64,
    pub deleted_at: Option<u64>,
}
