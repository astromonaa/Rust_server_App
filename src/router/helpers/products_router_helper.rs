use std::sync::Arc;
use axum::extract::{Path, State, Extension};
use axum::response::IntoResponse;
use crate::controllers::products_controller::ProductsController;
use crate::services::tokens_service::types::UserClaims;

pub async fn get_products_handler(
    State(products_controller): State<Arc<ProductsController>>,
    Extension(user): Extension<Option<UserClaims>>
) -> impl IntoResponse {
    products_controller.get_products(user).await
}

pub async fn get_product_item(
    State(products_controller): State<Arc<ProductsController>>,
    Path(product_id): Path<i32>,
    Extension(user): Extension<Option<UserClaims>>
) -> impl IntoResponse {
    products_controller.get_product_item(product_id, user).await
}