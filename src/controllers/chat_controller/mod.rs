use crate::db::entities::chat::Model as ChatModel;
use crate::db::entities::message::Model as MessageModel;
use crate::services::chat_service::ChatService;
use crate::services::tokens_service::types::UserClaims;
use crate::websocket::ws_helpers::get_browser_info;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

pub struct ChatController {
    service: ChatService,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateChatRequest {
    pub anonymous_user_id: Option<i32>,
    pub is_anonymous: bool,
    pub user_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub text: String,
    pub is_from_user: bool,
}

#[derive(Serialize)]
pub struct ChatResponse {
    pub chat: Vec<ChatModel>,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: MessageModel,
}

#[derive(Serialize)]
pub struct ChatWithMessagesResponse {
    pub chats: Vec<ChatModel>,
    pub messages: Vec<MessageModel>,
}

impl ChatController {
    pub fn new(service: ChatService) -> Self {
        Self { service }
    }

    pub async fn create_or_get_chat(
        &self,
        body: CreateChatRequest,
    ) -> Result<Json<ChatResponse>, (StatusCode, String)> {
        let chats = self
            .service
            .get_chats_by_user_and_anonymous(body.user_id, body.anonymous_user_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let auth_user_chat_doesnt_exist = body.user_id.is_some() && chats.iter().find(|c| c.user_id == body.user_id).is_none();

        if !chats.is_empty() && !auth_user_chat_doesnt_exist {
            return Ok(Json(ChatResponse {
                chat: chats,
            }));
        }

        let chat = self
            .service
            .create_chat(body.user_id, body.anonymous_user_id, body.is_anonymous)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        Ok(Json(ChatResponse { chat: vec![chat] }))
    }

    pub async fn send_message(
        &self,
        chat_id: i32,
        body: SendMessageRequest,
    ) -> Result<Json<MessageResponse>, (StatusCode, String)> {
        let message = self
            .service
            .send_message(chat_id, body.text, body.is_from_user)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(MessageResponse { message }))
    }


    pub async fn get_user_chat_with_messages(
        &self,
        user_id: Option<i32>,
        anonymous_user_id: Option<i32>,
        is_anonymous: bool,
    ) -> Result<Json<ChatWithMessagesResponse>, (StatusCode, String)> {
        // Get or create chat
        let chat_response = self
            .create_or_get_chat(CreateChatRequest {
                user_id,
                anonymous_user_id,
                is_anonymous,
            })
            .await?;

        let chats = chat_response.0.chat;
        let chat_ids: Vec<i32> = chats.iter().map(|c| c.id).collect();

        // Get messages for all chats in one query
        let messages = self
            .service
            .get_messages_by_chat_ids(chat_ids)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(ChatWithMessagesResponse { 
            chats,
            messages 
        }))
    }
}
