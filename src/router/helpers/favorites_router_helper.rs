use std::sync::Arc;
use axum::extract::{Path, State, Extension};
use axum::response::IntoResponse;
use crate::controllers::favorites_controller::FavoritesController;
use crate::services::tokens_service::types::UserClaims;

pub async fn add_to_favorites(
    State(favorites_controller): State<Arc<FavoritesController>>,
    Path(product_id): Path<i32>,
    Extension(user): Extension<Option<UserClaims>>
) -> impl IntoResponse {
    favorites_controller.add_to_favorites(user, product_id).await
}

pub async fn get_favorites_list(
    State(favorites_controller): State<Arc<FavoritesController>>,
    Extension(user): Extension<Option<UserClaims>>
) -> impl IntoResponse {
    favorites_controller.get_favorites_list(user).await
}