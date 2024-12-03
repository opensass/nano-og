use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreOGRequest {
    pub token: String,
    pub title: String,
    pub description: String,
    pub site_name: String,
    pub image_url: String,
    pub author: String,
    pub locale: String,
    pub twitter_card: String,
    pub twitter_site: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateOGContentRequest {
    pub og_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub site_name: Option<String>,
    pub image_url: Option<String>,
    pub author: Option<String>,
    pub locale: Option<String>,
    pub twitter_card: Option<String>,
    pub twitter_site: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetOGsForUserRequest {
    pub token: String,
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
