use std::sync::Arc;
use axum::middleware::from_fn;
use axum::Router;
use sea_orm::DatabaseConnection;
use axum::routing::{get, post};
use crate::controllers::favorites_controller::FavoritesController;
use crate::middleware::get_user::get_user_middleware;
use crate::router::{build_favorites_service, build_product_service};
use crate::router::helpers::favorites_router_helper;
use crate::services::fav_and_product_service::FavAndProductService;
use crate::services::favorites_service::FavoritesService;

fn build_fav_and_product_service(connection: &Arc<DatabaseConnection>, fav_service: FavoritesService) -> FavAndProductService {
    FavAndProductService::new(fav_service, build_product_service(connection))
}

fn build_controller(connection: &Arc<DatabaseConnection>) -> Arc<FavoritesController> {
    let fav_service = build_favorites_service(connection);
    Arc::new(FavoritesController::new(fav_service.clone(), build_fav_and_product_service(connection, fav_service)))
}

pub fn setup_router(connection: DatabaseConnection) -> anyhow::Result<Router> {
    let connection = Arc::new(connection);


    let router = Router::new()
        .route("/", get(favorites_router_helper::get_favorites_list).route_layer(from_fn(get_user_middleware)))
        .route("/:product_id", post(favorites_router_helper::add_to_favorites).route_layer(from_fn(get_user_middleware)))
        .with_state(build_controller(&connection));

    Ok(router)
}