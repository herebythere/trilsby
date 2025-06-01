use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Tags {
    id: u64,
    kind: String,
    deleted_at: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub TagToBookmark {
    id: u64,
    tag_id: u64,
    bookmark_id: u64,
    deleted_at: u64,
}
