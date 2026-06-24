use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Bookmark {
    pub id: u64,
    pub url: String,
    pub deleted_at: Option<u64>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct TagToBookmark {
    pub id: u64,
    pub tag_id: u64,
    pub bookmark_id: u64,
    pub updated_at: u64,
    pub deleted_at: Option<u64>,
}
