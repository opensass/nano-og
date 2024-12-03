#![allow(unused)]
#![allow(dead_code)]

use bson::doc;
use dioxus::prelude::*;
use dioxus_logger::tracing;

#[cfg(feature = "server")]
use crate::db::get_client;
use crate::server::auth::controller::auth;
use crate::server::common::response::SuccessResponse;
use crate::server::conversation::model::Conversation;
use crate::server::conversation::model::Message;
use crate::server::conversation::request::CreateConversationRequest;
use crate::server::conversation::request::GetConversationsRequest;
use crate::server::conversation::request::GetMessagesRequest;
use crate::server::conversation::request::SendQueryRequest;
use crate::server::conversation::response::ConversationResponse;
use crate::server::conversation::response::ConversationsListResponse;
use crate::server::conversation::response::MessageResponse;
use crate::server::conversation::response::MessagesListResponse;
use crate::server::og::model::OG;
use bson::oid::ObjectId;
use chrono::prelude::*;
use futures_util::TryStreamExt;
use std::env;

#[server]
pub async fn create_conversation(
    req: CreateConversationRequest,
) -> Result<ConversationResponse, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;
    let db_client = get_client().await;
    let db = db_client
        .database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let conversation_collection = db.collection::<Conversation>("conversations");

    let og_id = ObjectId::parse_str(&req.og_id).map_err(|_| ServerFnError::new("Invalid og ID"))?;

    let conversation = Conversation {
        id: ObjectId::new(),
        user: user.id,
        og: og_id,
        title: req.title,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    conversation_collection
        .insert_one(conversation.clone())
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;
    Ok(ConversationResponse {
        status: "success".to_string(),
        data: conversation,
    })
}

#[server]
pub async fn get_conversations(
    req: GetConversationsRequest,
) -> Result<ConversationsListResponse, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let db_client = get_client().await;
    let db = db_client
        .database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let conversation_collection = db.collection::<Conversation>("conversations");

    let filter = doc! {"user": user.id, "og": req.og_id};
    let cursor = conversation_collection
        .find(filter)
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;
    let conversations: Vec<Conversation> = cursor
        .try_collect()
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;

    Ok(ConversationsListResponse {
        status: "success".to_string(),
        data: conversations,
    })
}

#[server]
pub async fn save_message_to_db(message: Message) -> Result<(), ServerFnError> {
    let db_client = get_client().await;
    let db = db_client
        .database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let messages_collection = db.collection::<Message>("messages");

    messages_collection
        .insert_one(message)
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;
    Ok(())
}

#[server]
pub async fn get_messages(req: GetMessagesRequest) -> Result<MessagesListResponse, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let db_client = get_client().await;
    let db = db_client
        .database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let messages_collection = db.collection::<Message>("messages");

    let filter = doc! {"conversation": req.conversation_id};
    let cursor = messages_collection
        .find(filter)
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;
    let messages: Vec<Message> = cursor
        .try_collect()
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;

    Ok(MessagesListResponse {
        status: "success".to_string(),
        data: messages,
    })
}
