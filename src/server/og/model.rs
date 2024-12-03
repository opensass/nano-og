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
    pub description: String,
    #[serde(rename = "siteName")]
    pub site_name: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub author: String,
    pub locale: String,
    #[serde(rename = "twitterCard")]
    pub twitter_card: String,
    #[serde(rename = "twitterSite")]
    pub twitter_site: String,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
