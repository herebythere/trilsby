use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct PersonalBookmark {
    pub id: u64,
    pub people_id: u64,
    pub url: String,
    pub updated_at: u64,
    pub deleted_at: Option<u64>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct TagToPersonalBookmark {
    pub id: u64,
    pub tag_id: u64,
    pub personal_bookmark_id: u64,
    pub people_id: u64,
    pub updated_at: u64,
    pub deleted_at: Option<u64>,
}
