use crate::controllers::chat_controller::ChatController;
use crate::controllers::user_controller::UserController;
use crate::middleware::get_user::get_user_middleware;
use crate::router::helpers::chats_router_helper;
use crate::router::user_router::build_user_controller;
use crate::websocket::ws_helpers::build_chat_controller;
use axum::middleware::from_fn;
use axum::routing::{get, post};
use axum::{Extension, Router};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tower_cookies::{CookieManagerLayer, Cookies};

#[derive(Clone)]
pub struct ChatRouterState {
    pub chat_controller: Arc<ChatController>,
    pub user_controller: Arc<UserController>,
}

pub fn setup_router(connection: DatabaseConnection) -> anyhow::Result<Router> {
    let connection = Arc::new(connection);
    let cookies = Cookies::default();

    let state = ChatRouterState {
        chat_controller: build_chat_controller(&connection),
        user_controller: build_user_controller(&connection),
    };

    let router = Router::new()
        .route("/", get(chats_router_helper::get_user_chat).route_layer(from_fn(get_user_middleware)))
        .with_state(state)
        .layer(CookieManagerLayer::new())
        .layer(Extension(cookies));
    Ok(router)
}
