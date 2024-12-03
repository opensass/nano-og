use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateConversationRequest {
    pub token: String,
    pub og_id: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetConversationsRequest {
    pub token: String,
    pub og_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetMessagesRequest {
    pub token: String,
    pub conversation_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendQueryRequest {
    pub conversation_id: ObjectId,
    pub query: String,
    pub og: String,
    pub detail: String,
    pub model: String,
    pub token: String,
}
