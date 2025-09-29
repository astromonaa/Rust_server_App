use std::sync::Arc;
use axum::{Extension, Router};
use axum::routing::{get, post};
use anyhow::Result;
use sea_orm::DatabaseConnection;
use tower_cookies::{CookieManagerLayer, Cookies};
use crate::controllers::user_controller::UserController;
use crate::repository::user_repository::DbUserRepository;
use crate::router::{build_cart_service, build_favorites_service};
use crate::services::user_service::UserService;
use crate::router::helpers::user_router_helper;
use crate::services::mail_service::MailService;
fn build_mail_service() -> MailService {
    MailService::new().expect("Failed to create mail service")
}

fn build_user_service(connection: &Arc<DatabaseConnection>) -> UserService {
    let user_repository = DbUserRepository::new(Arc::clone(connection));
    let user_service = UserService::new(
        user_repository,
        build_cart_service(connection),
        build_favorites_service(connection),
        build_mail_service()
    );
    user_service
}

fn build_controller(connection: &Arc<DatabaseConnection>) -> Arc<UserController> {
    Arc::new(UserController::new(build_user_service(connection)))
}

pub fn setup_router(connection: DatabaseConnection) -> Result<Router> {
    let connection = Arc::new(connection);

    let cookies = Cookies::default();

    let router = Router::new()
        .route("/", get(user_router_helper::get_users))
        .route("/registration", post(user_router_helper::registration))
        .route("/activate/:activation_link", get(user_router_helper::activate_user))
        .route("/login", post(user_router_helper::login))
        .route("/auth", get(user_router_helper::auth))
        .route("/logout", post(user_router_helper::logout))
        .with_state(build_controller(&connection))
        .layer(CookieManagerLayer::new())
        .layer(Extension(cookies));

    Ok(router)
}