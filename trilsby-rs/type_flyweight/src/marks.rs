use serde::{Deserialize, Serialize};

// Mark like on a map or like a flagpost
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Mark {
    pub id: u64,
    pub url: String,
    pub last_successful_request_at: u64,
    pub requested_at: u64,
    pub request_status: String,
    pub deleted_at: Option<u64>,
}

// could be derived into maps you can bounce around in

// heres a website. here's the links it's got the articles the images.

// Like scattered on a screen? navigable again, no nothing just flat?

// is that what im building?

// A need to update and stuff
// If id has been so long out of date, remove from graph

pub struct MarkReceipt {
    pub mark_id: u64,
    pub url: String,
    pub last_successful_request_at: u64,
    pub request_status: String,
    pub requested_at: u64,
}

pub struct Edge {
    pub origin_mark_id: u64,
    pub destination_mark_id: u64,
    pub deleted_at: Option<u64>,
}
