use crate::controllers::chat_controller::{ChatController, ChatResponse, CreateChatRequest};
use crate::controllers::user_controller::UserController;
use crate::db::entities::anonymous_user::Model as AnonymousUserModel;
use crate::repository::chat_repository::DBChatRepository;
use crate::services::chat_service::ChatService;
use crate::services::tokens_service::types::UserClaims;
use axum::extract::Request;
use axum::http::header::USER_AGENT;
use axum::http::StatusCode;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;


#[derive(Debug, Deserialize, Serialize)]
pub struct BrowserInfo {
    pub user_agent: String,
    pub ip_address: String,
    pub device_fingerprint: String,
}

pub fn get_browser_info(request: Request, addr: SocketAddr) -> anyhow::Result<BrowserInfo> {
    let user_agent = request
        .headers()
        .get(USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .unwrap()
        .to_string();

    let ip_address = addr.ip().to_string();

    let device_fingerprint = request
        .uri()
        .query()
        .and_then(|q| {
            url::form_urlencoded::parse(q.as_bytes())
                .find(|(key, _)| key == "fingerprint")
                .map(|(_, value)| value.to_string())
        })
        .ok_or_else(|| anyhow::Error::msg("No fingerprint found"))?;

    Ok(BrowserInfo {
        user_agent,
        ip_address,
        device_fingerprint,
    })
}

pub fn build_chat_controller(connection: &Arc<DatabaseConnection>) -> Arc<ChatController> {
    Arc::new(ChatController::new(ChatService::new(
        DBChatRepository::new(Arc::clone(connection)),
    )))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WebSocketMessage {
    pub data: String,
    #[serde(rename = "roomId")]
    pub room_id: String,
    pub channel: String,
}

/// Create or get anonymous user
pub async fn create_or_get_anonymous_user(
    user_controller: &Arc<UserController>,
    browser_info: BrowserInfo,
) -> Result<AnonymousUserModel, (StatusCode, String)> {
    let response = user_controller
        .create_or_get_anonymous_user(browser_info)
        .await?;

    Ok(response.0)
}

/// Get user IDs based on authentication status
pub async fn get_user_ids(
    user: Option<UserClaims>,
    user_controller: &Arc<UserController>,
    browser_info: BrowserInfo,
) -> anyhow::Result<(Option<i32>, Option<i32>, bool)> {
    let anonymous_user = create_or_get_anonymous_user(user_controller, browser_info)
        .await
        .map_err(|(status, msg)| {
            anyhow::anyhow!("Failed to create anonymous user: {} - {}", status, msg)
        })?;

    if let Some(u) = user {
        Ok((Some(u.id), Some(anonymous_user.id), false))
    } else {
        Ok((None, Some(anonymous_user.id), true))
    }
}

/// Create or get chat
pub async fn create_or_get_chat(
    chat_controller: &Arc<ChatController>,
    user_id: Option<i32>,
    anonymous_user_id: Option<i32>,
    is_anonymous: bool,
) -> anyhow::Result<ChatResponse> {
    let response = chat_controller
        .create_or_get_chat(CreateChatRequest {
            user_id,
            anonymous_user_id,
            is_anonymous,
        })
        .await
        .map_err(|(status, msg)| anyhow::anyhow!("Failed to create/get chat: {} - {}", status, msg))?;

    Ok(response.0)
}

/// Parse WebSocket message from text
pub fn parse_ws_message(text: &str) -> Result<WebSocketMessage, serde_json::Error> {
    serde_json::from_str(text)
}

use crate::controllers::chat_controller::SendMessageRequest;
use axum::extract::ws::Message;

pub enum MessageProcessResult {
    Continue,
    Stop,
    Message(SendMessageRequest, Message),
}

/// Process incoming WebSocket message and convert to SendMessageRequest
pub fn process_incoming_message(msg: Result<Message, axum::Error>) -> MessageProcessResult {
    let msg = match msg {
        Ok(m) => m,
        Err(_) => return MessageProcessResult::Stop,
    };

    let text = match msg.to_text() {
        Ok(t) => t,
        Err(_) => return MessageProcessResult::Continue,
    };

    let ws_message = match parse_ws_message(text) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to parse message: {}", e);
            return MessageProcessResult::Continue;
        }
    };

    let request = SendMessageRequest {
        text: ws_message.data,
        is_from_user: true,
    };

    MessageProcessResult::Message(request, msg.clone())
}
