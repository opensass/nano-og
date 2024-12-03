#![allow(unused)]
#![allow(dead_code)]

use bson::doc;
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::server::auth::controller::auth;
use crate::server::common::response::SuccessResponse;
use crate::server::og::model::Detail;
use crate::server::og::model::OG;
use crate::server::og::request::AIRequest;
use crate::server::og::request::CompleteOGRequest;
use crate::server::og::request::GenerateDetailContentRequest;
use crate::server::og::request::GenerateOGRequest;
use crate::server::og::request::GetDetailContentRequest;
use crate::server::og::request::GetOGForUserRequest;
use crate::server::og::request::GetOGsForUserRequest;
use crate::server::og::request::StoreOGRequest;
use crate::server::og::request::UpdateOGContentRequest;
use crate::server::og::response::GenerateOGOutlineResponse;
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

    let photo_url = fetch_cover(req.title.to_string()).await?;

    let new_og = OG {
        id: ObjectId::new(),
        user: user.id,
        title: req.title,
        subtitle: Some(req.subtitle),
        og_type: req.og_type,
        cover: photo_url,
        completed: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    og_collection.insert_one(new_og.clone()).await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: OGResponse { id: new_og.id },
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
pub async fn update_detail_content(
    req: UpdateOGContentRequest,
) -> Result<SuccessResponse<String>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let og_collection = db.collection::<OG>("ogs");

    let og_id = ObjectId::parse_str(&req.og_id).map_err(|_| ServerFnError::new("Invalid og ID"))?;

    og_collection
        .update_one(
            doc! { "_id": og_id },
            doc! { "$set": { "content": req.new_content, "updatedAt": Utc::now() } },
        )
        .await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: "OG updated successfully".into(),
    })
}

#[server]
pub async fn complete_og(req: CompleteOGRequest) -> Result<SuccessResponse<String>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let og_collection = db.collection::<OG>("ogs");

    og_collection
        .update_one(
            doc! { "_id": req.og_id },
            doc! { "$set": { "completed": true, "updatedAt": Utc::now() } },
        )
        .await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: "OG marked as completed".into(),
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

    let ogs = og_collection
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
pub async fn generate_og_outline(
    req: GenerateOGRequest,
) -> Result<SuccessResponse<GenerateOGOutlineResponse>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let system_prompt = format!(
        "
        **System Prompt (SP):** You are an expert travel planner creating a structured, day-by-day og itinerary.
    
        **Prompt (P):** Create a travel outline titled '{title}' to the destination '{subtitle}'. The og should be planned with a main theme of '{title}', and presented in {language}. The itinerary should fit within a budget of {budget}. 
    
        Generate a day-by-day schedule for the og, including specific places to visit, activities, and an estimated time duration for each. Use a structured format for each day and activity.
    
        **Expected Format (EF):**
        ### Day [number]: [Day Title]
        #### Place [number]: [Place Name]
        **Estimated Duration:** [Duration] minutes
    
        * [Activity description]
        * [Additional information as needed]
    
        **Roleplay (RP):** As a travel planner, make the plan engaging and realistic.
        ",
        title = req.title,
        subtitle = req.subtitle,
        budget = req.subtopics,
        language = req.language,
    );

    let outline = req.outline;

    let db_client = get_client().await;
    let db = db_client
        .database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let og_collection = db.collection::<OG>("ogs");

    let photo_url = fetch_cover(req.title.clone()).await?;

    let og = OG {
        id: ObjectId::new(),
        user: user.id,
        title: req.title.clone(),
        subtitle: Some(req.subtitle.clone()),
        og_type: Some(req.title.clone()),
        completed: false,
        cover: photo_url,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    og_collection.insert_one(og.clone()).await?;

    let details = parse_outline(outline.clone(), og.id, req.language)?;

    let details_collection = db.collection::<Detail>("details");
    details_collection.insert_many(details.clone()).await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: GenerateOGOutlineResponse {
            og: og.clone(),
            details: details.clone(),
        },
    })
}

fn parse_outline(
    outline: String,
    og_id: ObjectId,
    language: String,
) -> Result<Vec<Detail>, ServerFnError> {
    let mut details = Vec::new();

    let day_re = Regex::new(r"### Day (\d+): (.*?)\n").unwrap();
    let place_re =
        Regex::new(r"#### Place (\d+): (.*?)\n\*\*Estimated Duration:\*\* (\d+) minutes").unwrap();
    let activity_re = Regex::new(r"\* (.+)").unwrap();

    let mut current_position = 0;

    while let Some(day_caps) = day_re.captures(&outline[current_position..]) {
        let day_number: i32 = day_caps[1].parse().unwrap_or(1);
        let day_title = &day_caps[2];

        let day_start = current_position + day_caps.get(0).unwrap().end();
        let next_day_pos = day_re
            .find_at(&outline, day_start)
            .map_or(outline.len(), |m| m.start());

        let day_content = &outline[day_start..next_day_pos];
        current_position = next_day_pos;

        let mut place_pos = 0;
        while let Some(place_caps) = place_re.captures(&day_content[place_pos..]) {
            let place_number: i32 = place_caps[1].parse().unwrap_or(1);
            let place_name = &place_caps[2];
            let estimated_duration = place_caps[3].parse().unwrap_or(0);

            let place_start = place_pos + place_caps.get(0).unwrap().end();
            let next_place_pos = place_re
                .find_at(&day_content, place_start)
                .map_or(day_content.len(), |m| m.start());

            let place_content = &day_content[place_start..next_place_pos];
            place_pos = next_place_pos;

            let bullet_points = activity_re
                .find_iter(place_content)
                .map(|mat| mat.as_str().to_string())
                .collect::<Vec<String>>()
                .join("\n");

            details.push(Detail {
                id: ObjectId::new(),
                og_id,
                title: format!("Day {} - {}", day_number, day_title),
                html: format!("Place {}: {}\n{}", place_number, place_name, bullet_points),
                estimated_duration,
                language: language.clone(),
                completed: false,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            });
        }
    }

    Ok(details)
}

#[server]
pub async fn generate_detail_content(
    req: GenerateDetailContentRequest,
) -> Result<SuccessResponse<String>, ServerFnError> {
    let system_prompt = format!(
        "
        **System Prompt (SP):** You are writing detailed content for a og detail.

        **Prompt (P):** Write content for detail '{detail_title}' of the og '{og_title}' in {language}. Ensure clarity, detailed explanations, and structured markdown.

        **Expected Format (EF):**
        - detailed markdown format for this detail.

        **Roleplay (RP):** Provide as much educational content as possible.
        ",
        detail_title = req.detail_title,
        og_title = req.og_title,
        language = req.language,
    );

    let markdown = req.markdown;

    let content_prompt = format!(
        "Generate a comprehensive HTML-formatted og detail with examples, links and images, based on the outline: '{}' in {language}. \
        Each section should be structured with appropriate HTML tags, including <h1> for the main title, \
        <h2> for detail titles, <h3> for subheadings, and <p> for paragraphs. \
        Include well-organized, readable content that aligns with the og's outline, ensuring each section is \
        clear and logically flows from one to the next. Avoid markdown format entirely, and provide inline HTML styling \
        if necessary to enhance readability. The HTML content should be well-formatted, semantically correct, and \
        cover all relevant subtopics in depth to create an engaging reading experience. \
        Make sure to always return back with html formmatted text and not empty response.
        ",
        markdown.clone(),
        language = req.language,
    );

    let mut html = req.html;

    html = update_detail_content(UpdateOGContentRequest {
        og_id: req.detail_id.to_string(),
        new_content: html.clone(),
    })
    .await?
    .data;

    Ok(SuccessResponse {
        status: "success".into(),
        data: html,
    })
}

#[server]
pub async fn get_details_for_og(
    req: GetDetailContentRequest,
) -> Result<SuccessResponse<Vec<Detail>>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let og_collection = db.collection::<Detail>("details");

    let og_object_id =
        ObjectId::parse_str(&req.og_id).map_err(|_| ServerFnError::new("Invalid og ID"))?;

    let mut details = og_collection
        .find(doc! { "og_id": og_object_id })
        .await?
        .try_collect::<Vec<Detail>>()
        .await?;

    for detail in details.iter_mut() {
        if detail.html.is_empty() {
            let markdown_content = detail.html.clone();

            let content_prompt = format!(
                "Generate a comprehensive HTML-formatted og og with examples, links and images, based on the outline: '{}' in {language}. \
                Each section should be structured with appropriate HTML tags, including <h1> for the main title, \
                <h2> for og titles, <h3> for subheadings, and <p> for paragraphs. \
                Include well-organized, readable content that aligns with the og's outline, ensuring each section is \
                clear and logically flows from one to the next. Avoid markdown format entirely, and provide inline HTML styling \
                if necessary to enhance readability. The HTML content should be well-formatted, semantically correct, and \
                cover all relevant subtopics in depth to create an engaging reading experience. \
                Make sure to always return back with html formmatted text and not empty response.",
                markdown_content,
                language = detail.language,
            );

            let html_content = req.html.clone();

            og_collection
                .update_one(
                    doc! { "_id": detail.id },
                    doc! { "$set": { "html": html_content.clone(), "updatedAt": Utc::now() } },
                )
                .await?;

            detail.html = html_content;
        }
    }

    Ok(SuccessResponse {
        status: "success".into(),
        data: details,
    })
}
