use std::sync::Arc;
use axum::middleware::from_fn;
use axum::Router;
use axum::routing::{post, get};
use sea_orm::DatabaseConnection;
use crate::controllers::cart_controller::CartController;
use crate::middleware::get_user::get_user_middleware;
use crate::router::build_cart_service;
use crate::router::helpers::cart_router_helper;

fn build_controller(connection: &Arc<DatabaseConnection>) -> Arc<CartController> {
    Arc::new(CartController::new(build_cart_service(connection)))
}

pub fn setup_router(connection: DatabaseConnection) -> anyhow::Result<Router> {
    let connection = Arc::new(connection);


    let router = Router::new()
        .route("/", get(cart_router_helper::get_cart).route_layer(from_fn(get_user_middleware)))
        .route("/:product_id", post(cart_router_helper::add_to_cart).route_layer(from_fn(get_user_middleware)))
        .with_state(build_controller(&connection));

    Ok(router)
}