use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BookmarkList {
    pub id: u64,
    pub people_id: u64,
    pub title: String,
    pub description: Option<String>,
    pub image_uri: Option<String>,
    pub media_uri: Option<String>,
    pub deleted_at: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BookmarkListToBookmark {
    pub id: u64,
    pub bookmark_list_id: u64,
    pub bookmark_id: u64,
    pub people_id: u64,
    pub order_weight: u64,
    pub deleted_at: u64,
}
