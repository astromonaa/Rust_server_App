use crate::middleware::get_user_ws::get_user_middleware_ws;
use crate::router::user_router::build_user_controller;
use crate::services::tokens_service::types::UserClaims;
use axum::http::StatusCode;
use axum::middleware::from_fn;
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade, Message},
        ConnectInfo, Request,
    },
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use sea_orm::DatabaseConnection;
use std::net::SocketAddr;
use std::sync::Arc;

pub mod ws_helpers;
use ws_helpers::{
    build_chat_controller, get_browser_info, get_user_ids,
    create_or_get_chat, process_incoming_message, MessageProcessResult, BrowserInfo,
};

pub async fn setup_ws_module(connection: DatabaseConnection) -> anyhow::Result<Router> {
    let connection = Arc::new(connection);

    let router = Router::new()
        .route(
            "/",
            get(ws_handler).route_layer(from_fn(get_user_middleware_ws)),
        )
        .layer(Extension(connection));

    Ok(router)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(user): Extension<Option<UserClaims>>,
    Extension(connection): Extension<Arc<DatabaseConnection>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
) -> impl IntoResponse {
    let browser_info = match get_browser_info(request, addr) {
        Ok(i) => i,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    ws.on_upgrade(move |socket| async move {
        if let Err(e) = handle_socket(socket, browser_info, connection, user).await {
            eprintln!("WebSocket error: {:?}", e);
        }
    })
}

async fn handle_socket(
    mut socket: WebSocket,
    browser_info: BrowserInfo,
    db: Arc<DatabaseConnection>,
    user: Option<UserClaims>,
) -> anyhow::Result<()> {
    let user_controller = build_user_controller(&db);
    let chat_controller = build_chat_controller(&db);

    let (user_id, anonymous_user_id, is_anonymous) =
        get_user_ids(user, &user_controller, browser_info).await?;

    let chat_response = create_or_get_chat(&chat_controller, user_id, anonymous_user_id, is_anonymous).await?;
    
    // Prioritize authenticated user chat over anonymous user chat
    let chat = chat_response.chat
        .iter()
        .find(|c| c.user_id.is_some())
        .or_else(|| chat_response.chat.iter().find(|c| c.anonymous_user_id.is_some()))
        .ok_or_else(|| anyhow::anyhow!("No chat found"))?
        .clone();

    while let Some(msg) = socket.recv().await {
        let (message, original_msg) = match process_incoming_message(msg) {
            MessageProcessResult::Stop => return Ok(()),
            MessageProcessResult::Continue => continue,
            MessageProcessResult::Message(m, original) => (m, original),
        };

        let _ = chat_controller
            .send_message(chat.id, message)
            .await
            .map_err(|(status, msg)| {
                anyhow::anyhow!("Failed to send message: {} - {}", status, msg)
            })?;

        // Echo message back to client
        if socket.send(original_msg).await.is_err() {
            return Ok(());
        }
    }

    Ok(())
}
