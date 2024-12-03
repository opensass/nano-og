use bson::oid::ObjectId;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OGResponse {
    pub id: ObjectId,
    pub user: ObjectId,
    pub title: String,
    pub description: String,
    pub site_name: String,
    pub image_url: String,
    pub author: String,
    pub locale: String,
    pub twitter_card: String,
    pub twitter_site: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateOGResponse {
    pub og: OGResponse,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AnalyticsData {
    pub engagement: EngagementStats,
    pub ai_usage: AIUsageStats,
    pub predictions: PredictiveStats,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EngagementStats {
    pub total_ogs: u64,
    pub avg_ogs_per_og: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AIUsageStats {
    pub total_ai_ogs: u64,
    pub avg_gen_time: f64,
    pub success_rate: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PredictiveStats {
    pub trending_genre: String,
    pub projected_growth: f64,
}
