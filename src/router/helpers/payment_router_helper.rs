use axum::extract::State;
use axum::{Extension, Json};
use crate::controllers::payment_controller::PaymentController;
use crate::services::tokens_service::types::UserClaims;

pub struct PaymentData {
    user_id: i32,
    order_id: i32,
}

pub async fn make_payment(
    State(_payment_controller): State<PaymentController>,
    Json(_body): Json<PaymentData>,
    Extension(_user): Extension<Option<UserClaims>>
) {
    // payment_controller.make_payment(body, user).await;
}