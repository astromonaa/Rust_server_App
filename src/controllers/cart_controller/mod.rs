use axum::http::StatusCode;
use axum::Json;
use crate::services::cart_service::CartService;
use crate::services::tokens_service::types::UserClaims;
use crate::db::entities::cart_product::{Model as CartProductModel};
use crate::DTO::cart_product_dto::{CartProductWithRelations, CartProductsWithTotals};

pub struct CartController {
    service: CartService,
}


impl CartController {
    pub fn new(service: CartService) -> Self {
        Self { service }
    }

    pub async fn add_to_cart(&self, user: Option<UserClaims>, product_id: i32, color: String, size: i32) -> Result<Json<CartProductModel>, (StatusCode, String)> {
        let cart_product = self.service.add_to_cart(user, product_id, color, size)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        Ok(Json(cart_product))
    }

    pub async fn get_cart(&self, user: Option<UserClaims>) -> Result<Json<CartProductsWithTotals>, (StatusCode, String)> {

        if user.is_none() {
            return Err((StatusCode::BAD_REQUEST, "User does not exist".to_string()));
        }

        let cart_products = self.service.get_cart(user)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        Ok(Json(cart_products))
    }
}