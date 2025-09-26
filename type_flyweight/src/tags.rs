use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TagKind {
    pub id: u64,
    pub kind: String,
    pub deleted_at: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Tag {
    id: u64,
    tag_id: u64,
    bookmark_id: u64,
    deleted_at: u64,
}
