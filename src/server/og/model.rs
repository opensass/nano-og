#![allow(non_snake_case)]

use bson::{oid::ObjectId, serde_helpers::chrono_datetime_as_bson_datetime};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct OG {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user: ObjectId,
    pub title: String,
    pub subtitle: Option<String>,
    #[serde(rename = "ogType")]
    pub og_type: Option<String>,
    #[serde(rename = "mainTopic")]
    pub completed: bool,
    pub cover: Option<String>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Detail {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub og_id: ObjectId,
    pub title: String,
    pub html: String,
    pub estimated_duration: u64,
    pub language: String,
    pub completed: bool,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
