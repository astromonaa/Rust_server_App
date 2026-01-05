use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::{ConnectInfo, Request, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use crate::router::chats_router::ChatRouterState;
use crate::services::tokens_service::types::UserClaims;
use crate::websocket::ws_helpers::{get_browser_info, get_user_ids};

pub async fn get_user_chat(
    State(state): State<ChatRouterState>,
    Extension(user): Extension<Option<UserClaims>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
) -> impl IntoResponse {
    let browser_info = match get_browser_info(request, addr) {
        Ok(i) => i,
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    };

    // Get user IDs (either authenticated user or anonymous user)
    let (user_id, anonymous_user_id, is_anonymous) = match get_user_ids(user, &state.user_controller, browser_info).await {
        Ok(ids) => ids,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    // Get chat with messages
    match state.chat_controller
        .get_user_chat_with_messages(user_id, anonymous_user_id, is_anonymous)
        .await
    {
        Ok(response) => response.into_response(),
        Err((status, msg)) => (status, msg).into_response(),
    }
}