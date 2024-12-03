#![allow(unused)]
#![allow(dead_code)]

use bson::doc;
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::server::auth::controller::auth;
use crate::server::common::response::SuccessResponse;
use crate::server::og::model::OG;
use crate::server::og::request::AIRequest;
use crate::server::og::request::GetOGForUserRequest;
use crate::server::og::request::GetOGsForUserRequest;
use crate::server::og::request::StoreOGRequest;
use crate::server::og::request::UpdateOGContentRequest;
use crate::server::og::response::GenerateOGResponse;
use crate::server::og::response::OGResponse;
use crate::server::og::response::{AIUsageStats, AnalyticsData, EngagementStats, PredictiveStats};
use std::env;

use bson::oid::ObjectId;
use chrono::prelude::*;
use futures_util::StreamExt;
use futures_util::TryStreamExt;
use regex::Regex;
#[cfg(feature = "server")]
use {
    crate::db::get_client,
    crate::unsplash::get_unsplash_client,
    http_api_isahc_client::{Client as _, IsahcClient},
    rand::thread_rng,
    rand::Rng,
    unsplash_api::endpoints::common::EndpointRet,
    unsplash_api::endpoints::search_photos::SearchPhotos,
    unsplash_api::endpoints::search_photos::SearchPhotosResponseBodyOkJson,
    unsplash_api::objects::pagination::Pagination,
    unsplash_api::objects::rate_limiting::RateLimiting,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GooglePlacesResponse {
    pub predictions: Vec<Prediction>,
}

#[derive(Serialize, Deserialize)]
pub struct Prediction {
    pub description: String,
    pub place_id: String,
}

#[server]
pub async fn store_og(req: StoreOGRequest) -> Result<SuccessResponse<OGResponse>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let og_collection = db.collection::<OG>("ogs");

    // if req.image_url.is_empty() {
    //     let photo_url = fetch_cover(req.title.clone()).await?;
    // }
    let new_og = OG {
        id: ObjectId::new(),
        user: user.id,
        title: req.title,
        description: req.description,
        site_name: req.site_name,
        image_url: req.image_url,
        author: req.author,
        locale: req.locale,
        twitter_card: req.twitter_card,
        twitter_site: req.twitter_site,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    og_collection.insert_one(new_og.clone()).await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: OGResponse {
            id: new_og.id,
            user: new_og.user,
            title: new_og.title,
            description: new_og.description,
            site_name: new_og.site_name,
            image_url: new_og.image_url,
            author: new_og.author,
            locale: new_og.locale,
            twitter_card: new_og.twitter_card,
            twitter_site: new_og.twitter_site,
            created_at: new_og.created_at,
            updated_at: new_og.updated_at,
        },
    })
}

#[server]
pub async fn fetch_cover(topic: String) -> Result<Option<String>, ServerFnError> {
    let client = get_unsplash_client().await.lock().await;

    let search_photos = SearchPhotos::new(
        &env::var("UNSPLASH_API_KEY").expect("UNSPLASH_API_KEY must be set."),
        topic,
    );

    let response: EndpointRet<(SearchPhotosResponseBodyOkJson, Pagination, RateLimiting)> =
        client.respond_endpoint(&search_photos).await?;

    let mut extracted_data = Vec::new();

    if let EndpointRet::Ok((ok_json, _pagination, _rate_limiting)) = response {
        for photo in ok_json.results {
            let image_url = photo.urls.regular.to_string();

            extracted_data.push(image_url);
        }
    } else {
        tracing::error!("Unexpected response type");
    }

    if extracted_data.is_empty() {
        return Ok(None);
    }

    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..extracted_data.len());
    Ok(Some(extracted_data[random_index].clone()))
}

#[server]
pub async fn update_og(
    req: UpdateOGContentRequest,
) -> Result<SuccessResponse<String>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let og_collection = db.collection::<OG>("ogs");

    let og_id = ObjectId::parse_str(&req.og_id).map_err(|_| ServerFnError::new("Invalid og ID"))?;

    let mut updates = doc! {};
    if let Some(title) = req.title {
        updates.insert("title", title);
    }
    if let Some(description) = req.description {
        updates.insert("description", description);
    }
    if let Some(site_name) = req.site_name {
        updates.insert("siteName", site_name);
    }
    if let Some(image_url) = req.image_url {
        updates.insert("imageUrl", image_url);
    }
    if let Some(author) = req.author {
        updates.insert("author", author);
    }
    if let Some(locale) = req.locale {
        updates.insert("locale", locale);
    }
    if let Some(twitter_card) = req.twitter_card {
        updates.insert("twitterCard", twitter_card);
    }
    if let Some(twitter_site) = req.twitter_site {
        updates.insert("twitterSite", twitter_site);
    }

    updates.insert("updatedAt", Utc::now());

    og_collection
        .update_one(doc! { "_id": og_id }, doc! { "$set": updates })
        .await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: "OG updated successfully".into(),
    })
}

#[server]
pub async fn get_ogs_for_user(
    req: GetOGsForUserRequest,
) -> Result<SuccessResponse<Vec<OG>>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let og_collection = db.collection::<OG>("ogs");

    let ogs: Vec<OG> = og_collection
        .find(doc! { "user": user.id })
        .await?
        .try_collect()
        .await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: ogs,
    })
}

#[server]
pub async fn get_og_for_user(
    req: GetOGForUserRequest,
) -> Result<SuccessResponse<OG>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let og_collection = db.collection::<OG>("ogs");

    let og_id = ObjectId::parse_str(&req.og_id).map_err(|_| ServerFnError::new("Invalid og ID"))?;

    let og = og_collection
        .find_one(doc! { "_id": og_id, "user": user.id })
        .await?
        .ok_or(ServerFnError::new("OG not found"))?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: og,
    })
}
