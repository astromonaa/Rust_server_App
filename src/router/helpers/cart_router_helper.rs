use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use serde::Deserialize;
use crate::controllers::cart_controller::CartController;
use crate::services::tokens_service::types::UserClaims;


#[derive(Deserialize)]
pub struct AddToCartPayload {
    pub size: i32,
    pub color: String,
}


pub async fn add_to_cart(
    State(cart_controller): State<Arc<CartController>>,
    Path(product_id): Path<i32>,
    Extension(user): Extension<Option<UserClaims>>,
    Json(payload): Json<AddToCartPayload>
) -> impl IntoResponse {
    cart_controller.add_to_cart(user, product_id, payload.color, payload.size).await
}

pub async fn get_cart(
    State(cart_controller): State<Arc<CartController>>,
    Extension(user): Extension<Option<UserClaims>>,
) -> impl IntoResponse {
    cart_controller.get_cart(user).await
}