use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use crate::router::helpers::order_router_helper::OrderData;
use crate::services::order_service::OrderService;
use crate::services::tokens_service::types::UserClaims;

pub struct OrderController {
    service: OrderService
}

#[derive(Serialize)]
pub struct OrderResponse {
    pub message: String,
    pub order_id: Option<i32>,
}

impl OrderController {
    pub fn new(service: OrderService) -> OrderController {
        OrderController {
            service
        }
    }

    pub async fn create_order(&self, _body: OrderData, user: Option<UserClaims>) -> Result<Json<OrderResponse>, (StatusCode, String)> {
        if user.is_none() {
            return Err((StatusCode::BAD_REQUEST, "User does not exist".to_string()));
        }

        let order = self.service.create_order(user.unwrap().id, _body).await;

        Ok(Json(OrderResponse {
            message: "Order created successfully".to_string(),
            order_id: Some(1), // Placeholder
        }))
    }

    pub async fn get_orders(&self, user: Option<UserClaims>) -> Result<Json<OrderResponse>, (StatusCode, String)> {
        if user.is_none() {
            return Err((StatusCode::BAD_REQUEST, "User does not exist".to_string()));
        }

        Ok(Json(OrderResponse {
            message: "Orders retrieved successfully".to_string(),
            order_id: None,
        }))
    }

    pub async fn get_order_by_id(&self, order_id: i32, user: Option<UserClaims>) -> Result<Json<OrderResponse>, (StatusCode, String)> {
        if user.is_none() {
            return Err((StatusCode::BAD_REQUEST, "User does not exist".to_string()));
        }

        Ok(Json(OrderResponse {
            message: format!("Order {} retrieved successfully", order_id),
            order_id: Some(order_id),
        }))
    }

    pub async fn update_order_status(&self, order_id: i32, status: String, user: Option<UserClaims>) -> Result<Json<OrderResponse>, (StatusCode, String)> {
        if user.is_none() {
            return Err((StatusCode::BAD_REQUEST, "User does not exist".to_string()));
        }

        Ok(Json(OrderResponse {
            message: format!("Order {} status updated to {}", order_id, status),
            order_id: Some(order_id),
        }))
    }
}
