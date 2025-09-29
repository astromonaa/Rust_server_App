use axum::{Router, routing::get};
use sea_orm::{ DatabaseConnection};
use anyhow::Result;
use crate::controllers::products_controller::ProductsController;
use crate::repository::products_repository::DbProductsRepository;
use crate::services::products_service::ProductsService;
use std::sync::Arc;
use axum::middleware::from_fn;
use crate::middleware::get_user::get_user_middleware;
use crate::router::{build_cart_service, build_favorites_service};
use crate::router::helpers::products_router_helper;

pub fn setup_router(connection: DatabaseConnection) -> Result<Router> {

    let connection = Arc::new(connection);

    let products_repository = DbProductsRepository::new(Arc::clone(&connection));

    let products_service = ProductsService::new(
        products_repository,
        build_cart_service(&connection),
        build_favorites_service(&connection)
    );

    let products_controller = Arc::new(ProductsController::new(products_service));

    // Создание роутера и привязка маршрута к контроллеру
    let router = Router::new()
        .route("/", get(products_router_helper::get_products_handler).route_layer(from_fn(get_user_middleware)))
        .route("/:product_id", get(products_router_helper::get_product_item).route_layer(from_fn(get_user_middleware)))
        .with_state(products_controller);

    Ok(router)
}
