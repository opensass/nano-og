use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreOGRequest {
    pub token: String,
    pub title: String,
    pub subtitle: String,
    pub content: String,
    pub og_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateOGContentRequest {
    pub og_id: String,
    pub new_content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompleteOGRequest {
    pub og_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetOGsForUserRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateOGRequest {
    pub title: String,
    pub subtitle: String,
    pub token: String,
    pub model: String,
    pub outline: String,
    pub subtopics: u64,
    pub details: u64,
    pub language: String,
    pub max_length: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateDetailContentRequest {
    pub detail_title: String,
    pub detail_id: ObjectId,
    pub og_title: String,
    pub language: String,
    pub markdown: String,
    pub html: String,
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetOGForUserRequest {
    pub token: String,
    pub og_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AIRequest {
    pub token: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetDetailContentRequest {
    pub html: String,
    pub og_id: String,
}
