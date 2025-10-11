use axum::extract::State;
use axum::{Extension, Json};
use crate::controllers::payment_controller::PaymentController;
use crate::services::tokens_service::types::UserClaims;

pub struct PaymentData {
    user_id: i32,
    order_id: i32,
}

pub async fn make_payment(
    State(payment_controller): State<PaymentController>,
    Json(body): Json<PaymentData>,
    Extension(user): Extension<Option<UserClaims>>
) {
    // payment_controller.make_payment(body, user).await;
}