use axum::http::StatusCode;
use axum::Json;
use crate::services::favorites_service::FavoritesService;
use crate::services::tokens_service::types::UserClaims;
use crate::services::favorites_service::types::{FavoriteResult};
use crate::db::entities::product::{Model as ProductModel};
use crate::services::fav_and_product_service::FavAndProductService;

pub struct FavoritesController {
    service: FavoritesService,
    fav_and_product_service: FavAndProductService,
}


impl FavoritesController {
    pub fn new(service: FavoritesService, fav_and_product_service: FavAndProductService) -> Self {
        Self { service, fav_and_product_service }
    }

    pub async fn get_favorites_list(&self, user: Option<UserClaims>) -> Result<Json<Vec<ProductModel>>, (StatusCode, String)> {

        if user.is_none() {
            return Err((StatusCode::BAD_REQUEST, "User does not exist".to_string()));
        }

        let favorites_list = self.fav_and_product_service.get_favorites_list(user.unwrap().id)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
        Ok(Json(favorites_list))
    }

    pub async fn add_to_favorites(&self, user: Option<UserClaims>, product_id: i32) -> Result<Json<FavoriteResult>, (StatusCode, String)> {

        if user.is_none() {
            return Err((StatusCode::BAD_REQUEST, "User does not exist".to_string()));
        }

        let favorite = self.service.add_to_favorites(user, product_id)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
        Ok(Json(favorite))
    }
}