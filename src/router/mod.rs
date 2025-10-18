pub mod products_router;
mod user_router;
pub mod helpers;
mod favorites_router;
mod cart_router;
mod payment_router;
mod order_router;

use std::sync::Arc;
use axum::{ Router};
use anyhow::Result;
use sea_orm::DatabaseConnection;
use crate::db;
use crate::configuration::auth_configuration;
use crate::repository::cart_repository::DBCartRepository;
use crate::repository::favorites_repository::DBFavoritesRepository;
use crate::repository::products_repository::DbProductsRepository;
use crate::services::cart_service::CartService;
use crate::services::favorites_service::FavoritesService;
use crate::services::products_service::ProductsService;
use migration::{Migrator, MigratorTrait};

pub async fn setup_router() -> Result<Router> {
    // Подключение к базе данных
    let settings = auth_configuration::AuthConfiguration::load()?;
    let connection = db::get_connection_pool(settings).await;

    Migrator::up(&connection, None).await?;

    let router = Router::new()
        .nest("/products", products_router::setup_router(connection.clone())?)
        .nest("/users", user_router::setup_router(connection.clone())?)
        .nest("/favorites", favorites_router::setup_router(connection.clone())?)
        .nest("/cart", cart_router::setup_router(connection.clone())?)
        .nest("/orders", order_router::setup_router(connection.clone())?);
    Ok(router)
}



pub fn build_cart_service(connection: &Arc<DatabaseConnection>) -> CartService {
    let cart_repository = DBCartRepository::new(Arc::clone(connection));
    let cart_service = CartService::new(cart_repository);
    cart_service
}

pub fn build_favorites_service(connection: &Arc<DatabaseConnection>) -> FavoritesService {
    let favorites_repository = DBFavoritesRepository::new(Arc::clone(connection));
    let favorites_service = FavoritesService::new(favorites_repository);
    favorites_service
}

pub fn build_product_service(connection: &Arc<DatabaseConnection>) -> ProductsService {

    let products_repository = DbProductsRepository::new(Arc::clone(&connection));

    ProductsService::new(
        products_repository,
        build_cart_service(&connection),
        build_favorites_service(&connection)
    )
}
