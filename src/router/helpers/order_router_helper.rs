use std::net::SocketAddr;
use std::sync::Arc;
use axum::extract::{State, Path, ConnectInfo};
use axum::{Extension, Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use crate::controllers::order_controller::OrderController;
use crate::services::tokens_service::types::UserClaims;

#[derive(Serialize, Deserialize)]
pub struct OrderData {
    pub items: Vec<OrderItemData>,
    pub shipping_address: String,
    pub billing_address: String,
    pub payment_method: String,
    pub notes: Option<String>,
    pub currency: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Amount {
    pub value: String,
    pub currency: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationObject {
    pub id: String,
    pub status: String,
    pub amount: Amount,
    pub metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderNotificationData {
    pub event: String,
    pub object: NotificationObject,
}

#[derive(Serialize, Deserialize)]
pub struct OrderItemData {
    pub product_id: i32,
    pub quantity: i32,
    pub size: i32,
    pub color: String,
}

pub async fn create_order(
    State(order_controller): State<Arc<OrderController>>,
    Extension(user): Extension<Option<UserClaims>>,
    Json(body): Json<OrderData>
) -> impl IntoResponse {
    order_controller.create_order(body, user).await
}

pub async fn update_order_status(
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    State(order_controller): State<Arc<OrderController>>,
    Json(body): Json<OrderNotificationData>,
) -> impl IntoResponse {
    order_controller.update_order_status(body, peer).await
}

pub async fn get_orders(
    State(order_controller): State<Arc<OrderController>>,
    Extension(user): Extension<Option<UserClaims>>
) -> impl IntoResponse {
    order_controller.get_orders(user).await
}

pub async fn get_order_by_id(
    State(order_controller): State<Arc<OrderController>>,
    Extension(user): Extension<Option<UserClaims>>,
    Path(order_id): Path<i32>
) -> impl IntoResponse {
    order_controller.get_order_by_id(order_id, user).await
}