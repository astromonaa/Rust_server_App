use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::services::chat_service::ChatService;
use crate::services::tokens_service::types::UserClaims;
use crate::db::entities::chat::Model as ChatModel;
use crate::db::entities::message::Model as MessageModel;

pub struct ChatController {
    service: ChatService,
}

#[derive(Deserialize)]
pub struct CreateChatRequest {
    pub anonymous_user_id: i32,
    pub is_anonymous: bool,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
    pub is_from_user: bool,
}

#[derive(Serialize)]
pub struct ChatResponse {
    pub chat: ChatModel,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: MessageModel,
}

impl ChatController {
    pub fn new(service: ChatService) -> Self {
        Self { service }
    }

    pub async fn create_chat(&self, body: CreateChatRequest, user: Option<UserClaims>) -> Result<Json<ChatResponse>, (StatusCode, String)> {
        let user_id = user.map(|u| u.id);

        let chat = self.service.create_chat(user_id, body.anonymous_user_id, body.is_anonymous)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(ChatResponse { chat }))
    }

    pub async fn get_user_chats(&self, user: Option<UserClaims>) -> Result<Json<Vec<ChatModel>>, (StatusCode, String)> {
        if user.is_none() {
            return Err((StatusCode::UNAUTHORIZED, "User not authenticated".to_string()));
        }

        let chats = self.service.get_user_chats(user.unwrap().id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(chats))
    }

    pub async fn get_chat(&self, chat_id: i32, user: Option<UserClaims>) -> Result<Json<ChatModel>, (StatusCode, String)> {
        if user.is_none() {
            return Err((StatusCode::UNAUTHORIZED, "User not authenticated".to_string()));
        }

        let chat = self.service.get_chat(chat_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .ok_or((StatusCode::NOT_FOUND, "Chat not found".to_string()))?;

        Ok(Json(chat))
    }

    pub async fn send_message(&self, chat_id: i32, body: SendMessageRequest, user: Option<UserClaims>) -> Result<Json<MessageResponse>, (StatusCode, String)> {
        if user.is_none() {
            return Err((StatusCode::UNAUTHORIZED, "User not authenticated".to_string()));
        }

        let message = self.service.send_message(chat_id, body.content, body.is_from_user)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(MessageResponse { message }))
    }

    pub async fn get_chat_messages(&self, chat_id: i32, user: Option<UserClaims>) -> Result<Json<Vec<MessageModel>>, (StatusCode, String)> {
        if user.is_none() {
            return Err((StatusCode::UNAUTHORIZED, "User not authenticated".to_string()));
        }

        let messages = self.service.get_chat_messages(chat_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(messages))
    }

    pub async fn mark_messages_as_read(&self, chat_id: i32, user: Option<UserClaims>) -> Result<Json<()>, (StatusCode, String)> {
        if user.is_none() {
            return Err((StatusCode::UNAUTHORIZED, "User not authenticated".to_string()));
        }

        self.service.mark_messages_as_read(chat_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(()))
    }
}
