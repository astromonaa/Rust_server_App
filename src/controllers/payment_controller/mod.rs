use axum::http::StatusCode;
use crate::router::helpers::payment_router_helper::PaymentData;
use crate::services::payment_service::PaymentService;
use crate::services::tokens_service::types::UserClaims;

pub struct PaymentController {
    service: PaymentService
}

impl PaymentController {
    pub fn new(service: PaymentService) -> PaymentController {
        PaymentController {
            service
        }
    }

    pub async fn make_payment(&self, _body: PaymentData, _user: Option<UserClaims>) -> Result<String, (StatusCode, String)> {
        // if user.is_none() {
        //     return Err((StatusCode::BAD_REQUEST, "User does not exist".to_string()));
        // }

        Ok(String::from("Ok"))
    }
}