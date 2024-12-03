use crate::server::og::model::Detail;
use crate::server::og::model::OG;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OGResponse {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateOGOutlineResponse {
    pub details: Vec<Detail>,
    pub og: OG,
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
    pub total_details: u64,
    pub avg_details_per_og: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AIUsageStats {
    pub total_ai_details: u64,
    pub avg_gen_time: f64,
    pub success_rate: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PredictiveStats {
    pub trending_genre: String,
    pub projected_growth: f64,
}
