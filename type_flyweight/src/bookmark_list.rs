use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
struct BookmarkList {
    pub id: u64,
    pub people_id: u64,
    pub deleted_at: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct BookmarkListToBookmark {
    pub id: u64,
    pub people_id: u64,
    pub bookmark_id: u64,
    pub bookmark_list_id: u64,
    pub order_weight: u64,
    pub deleted_at: u64,
}
