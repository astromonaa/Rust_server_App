use crate::services::tokens_service::types::UserClaims;
use axum::middleware::from_fn;
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        ConnectInfo, Request,
    },
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use std::net::SocketAddr;
use axum::http::StatusCode;
use crate::middleware::get_user_ws::get_user_middleware_ws;

mod ws_helpers;
use ws_helpers::{BrowserInfo, get_browser_info};

pub async fn setup_ws_module() -> anyhow::Result<Router> {
    let router = Router::new().route(
        "/",
        get(ws_handler).route_layer(from_fn(get_user_middleware_ws)),
    );

    Ok(router)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(user): Extension<Option<UserClaims>>,
    // Add ConnectInfo to get the client's IP address
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    // Get the raw request to extract headers
    request: Request,
) -> impl IntoResponse {
    println!("user: {:?}", user);
    let info = match get_browser_info(request, addr) {
        Ok(i) => i,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    ws.on_upgrade(move |socket|{
        handle_socket(socket, info)
    })
}

async fn handle_socket(mut socket: WebSocket, browser_info: BrowserInfo) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            return;
        };

        println!("msg: {:?}", msg);

        if socket.send(msg).await.is_err() {
            return;
        };
    }
}
