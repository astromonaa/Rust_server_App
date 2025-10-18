use std::sync::Arc;
use axum::middleware::from_fn;
use axum::Router;
use axum::routing::{post, get, delete};
use sea_orm::DatabaseConnection;
use crate::controllers::cart_controller::CartController;
use crate::middleware::get_user::get_user_middleware;
use crate::router::{build_cart_service, build_product_service};
use crate::router::helpers::cart_router_helper;
use crate::services::cart_and_product_service::CartAndProductService;
use crate::services::cart_service::CartService;

fn build_cart_and_product_service(connection: &Arc<DatabaseConnection>, cart_service: CartService) -> CartAndProductService {
    CartAndProductService::new(cart_service, build_product_service(connection))
}

fn build_controller(connection: &Arc<DatabaseConnection>) -> Arc<CartController> {
    let cart_service = build_cart_service(connection);
    Arc::new(CartController::new(cart_service.clone(), build_cart_and_product_service(connection, cart_service)))
}

pub fn setup_router(connection: DatabaseConnection) -> anyhow::Result<Router> {
    let connection = Arc::new(connection);


    let router = Router::new()
        .route("/", get(cart_router_helper::get_cart).route_layer(from_fn(get_user_middleware)))
        .route("/:product_id", post(cart_router_helper::add_to_cart).route_layer(from_fn(get_user_middleware)))
        .route("/:product_id", delete(cart_router_helper::delete_from_cart).route_layer(from_fn(get_user_middleware)))
        .with_state(build_controller(&connection));

    Ok(router)
}