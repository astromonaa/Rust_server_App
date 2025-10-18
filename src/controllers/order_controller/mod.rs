mod order_status_strategies;

use std::net::SocketAddr;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use crate::controllers::order_controller::order_status_strategies::PaymentStrategyRegistry;
use crate::router::helpers::order_router_helper::{OrderData, OrderNotificationData};
use crate::services::order_service::OrderService;
use crate::services::order_service::types::OrderWithRelations;
use crate::services::tokens_service::types::UserClaims;

pub struct OrderController {
    service: OrderService
}

#[derive(Serialize)]
pub struct OrderResponse {
    pub message: String,
    pub confirmation_url: Option<String>,
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

        let confirmation_url = self.service.create_order(user.unwrap().id, _body).await.unwrap();

        Ok(Json(OrderResponse {
            message: "Order created successfully".to_string(),
            confirmation_url
        }))
    }

    pub async fn get_orders(&self, user: Option<UserClaims>) -> Result<Json<Vec<OrderWithRelations>>, (StatusCode, String)> {
        if user.is_none() {
            return Err((StatusCode::BAD_REQUEST, "User does not exist".to_string()));
        }

        let orders = self.service.get_user_orders(user.unwrap().id)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()))?;

        Ok(Json(orders))
    }

    pub async fn get_order_by_id(&self, _order_id: i32, user: Option<UserClaims>) -> Result<Json<()>, (StatusCode, String)> {
        if user.is_none() {
            return Err((StatusCode::BAD_REQUEST, "User does not exist".to_string()));
        }

        Ok(Json(()))
    }

    pub async fn update_order_status(&self, body: OrderNotificationData, peer: SocketAddr) -> Result<Json<()>, (StatusCode, String)> {

        let strategy_registry = PaymentStrategyRegistry::new(&self.service);
        let strategy = strategy_registry.get_strategy(&body.event);

        strategy.process(body, peer.ip())
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(()))
    }
}
