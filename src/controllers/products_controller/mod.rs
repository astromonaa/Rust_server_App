use axum::{response::Json, http::StatusCode};
use serde::Serialize;
use crate::DTO::product_dto::ProductWithRelations;
use crate::services::products_service::ProductsService;
use crate::services::tokens_service::types::UserClaims;

#[derive(Serialize)]
pub struct ProductsResponse {
    pub rows: Vec<ProductWithRelations>,
    pub count: usize,
}

pub struct ProductsController {
    service: ProductsService,
}

impl ProductsController {
    pub fn new(service: ProductsService) -> Self {
        Self { service }
    }
    pub async fn get_products(&self, user: Option<UserClaims>) ->  Result<Json<ProductsResponse>, (StatusCode, String)> {
        let rows = self.service.get_products(user)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let count = rows.len();

        Ok(Json(ProductsResponse { rows, count }))
    }

    pub async fn get_product_item(&self, product_id: i32, user: UserClaims) ->  Result<Json<ProductWithRelations>, (StatusCode, String)> {
        let product = self.service.get_product_item(product_id, user)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        Ok(Json(product))
    }

}