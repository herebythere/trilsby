use serde::{Deserialize, Serialize};

// Two structs based on the bookmark api.
// This is more about backups and syncing than jumping around.

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct BookmarkDetails {
    pub id: u64,
    pub r#type: String,
    pub index: Option<u32>,
    pub parent_id: Option<u64>,
    pub url: String,
    pub title: String,
    pub description: Option<String>, // not api standard but there
    pub updated_at: u64,
    pub deleted_at: Option<u64>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct FolderDetails {
    pub id: u64,
    pub r#type: String,
    pub index: Option<u32>,
    pub parent_id: Option<u64>,
    pub title: Option<String>,
    pub updated_at: u64,
    pub deleted_at: Option<u64>,
}
