use std::sync::Arc;
use axum::middleware::from_fn;
use axum::Router;
use sea_orm::DatabaseConnection;
use axum::routing::{get, post};
use crate::controllers::favorites_controller;
use crate::controllers::favorites_controller::FavoritesController;
use crate::middleware::get_user::get_user_middleware;
use crate::router::build_favorites_service;
use crate::router::helpers::favorites_router_helper;

fn build_controller(connection: &Arc<DatabaseConnection>) -> Arc<FavoritesController> {
    Arc::new(FavoritesController::new(build_favorites_service(connection)))
}

pub fn setup_router(connection: DatabaseConnection) -> anyhow::Result<Router> {
    let connection = Arc::new(connection);


    let router = Router::new()
        .route("/", get(favorites_router_helper::get_favorites_list).route_layer(from_fn(get_user_middleware)))
        .route("/:product_id", post(favorites_router_helper::add_to_favorites).route_layer(from_fn(get_user_middleware)))
        .with_state(build_controller(&connection));

    Ok(router)
}