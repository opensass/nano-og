#![allow(unused)]
#![allow(dead_code)]

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
use bson::doc;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;

use bson::oid::ObjectId;
use chrono::prelude::*;
use futures_util::StreamExt;
use futures_util::TryStreamExt;
use regex::Regex;
#[cfg(feature = "server")]
use {
    crate::db::get_client,
    crate::pinata::get_pinata_client,
    base64::{engine::general_purpose::STANDARD, Engine as _},
    pinata_sdk::PinByFile,
    rand::thread_rng,
    rand::Rng,
    std::fs::write,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadOGRequest {
    pub image_url: String,
}

#[server]
pub async fn store_og(req: StoreOGRequest) -> Result<SuccessResponse<OGResponse>, ServerFnError> {
    // HACK: Wait for image to upload
    tokio::time::sleep(tokio::time::Duration::from_secs(7)).await;

    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let og_collection = db.collection::<OG>("ogs");

    let mut cursor = og_collection
        .find(doc! {})
        .sort(doc! { "createdAt": -1 })
        .await
        .map_err(|_| ServerFnError::new("Failed to query OG collection"))?;

    if let Some(result) = cursor.next().await {
        match result {
            Ok(mut latest_og) => {
                latest_og.user = user.id;
                latest_og.title = req.title;
                latest_og.description = req.description;
                latest_og.site_name = req.site_name;
                latest_og.author = req.author;
                latest_og.locale = req.locale;
                latest_og.brand_url = req.brand_url;
                latest_og.twitter_card = req.twitter_card;
                latest_og.twitter_site = req.twitter_site;
                latest_og.updated_at = Utc::now();

                og_collection
                    .replace_one(doc! { "_id": latest_og.id }, latest_og.clone())
                    .await
                    .map_err(|_| ServerFnError::new("Failed to update OG document"))?;

                Ok(SuccessResponse {
                    status: "success".into(),
                    data: OGResponse {
                        id: latest_og.id,
                        user: latest_og.user,
                        title: latest_og.title.clone(),
                        description: latest_og.description.clone(),
                        site_name: latest_og.site_name.clone(),
                        image_url: latest_og.image_url.clone(),
                        author: latest_og.author.clone(),
                        locale: latest_og.locale.clone(),
                        twitter_card: latest_og.twitter_card.clone(),
                        twitter_site: latest_og.twitter_site.clone(),
                        created_at: latest_og.created_at,
                        updated_at: latest_og.updated_at,
                    },
                })
            }
            Err(_) => Err(ServerFnError::new("Failed to deserialize OG document")),
        }
    } else {
        Err(ServerFnError::new("No OG document found"))
    }
}

#[server(endpoint = "upload_og")]
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

#[server]
pub async fn upload_og(req: UploadOGRequest) -> Result<SuccessResponse<String>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let og_collection = db.collection::<OG>("ogs");

    let temp_file_path = "/tmp/uploaded_image.png";
    let decoded_data = STANDARD
        .decode(&req.image_url)
        .map_err(|_| ServerFnError::new("Invalid Base64 image data"))?;
    write(temp_file_path, decoded_data).map_err(|_| {
        ServerFnError::new("Failed to write Base64-decoded image to temporary file")
    })?;

    let pinata = get_pinata_client().await.lock().await;

    let pin_result = pinata
        .pin_file(PinByFile::new(temp_file_path))
        .await
        .map_err(|err| ServerFnError::new(format!("Pinata Error: {:?}", err)))?;

    let ipfs_url = format!("https://gateway.pinata.cloud/ipfs/{}", pin_result.ipfs_hash);

    let mut new_og = OG::default();
    new_og.image_url = ipfs_url.clone();

    new_og.created_at = Utc::now();

    og_collection.insert_one(new_og.clone()).await?;
    Ok(SuccessResponse {
        status: "success".into(),
        data: ipfs_url,
    })
}
