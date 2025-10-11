use std::sync::Arc;
use axum::{Extension, Router};
use axum::routing::{get, post};
use sea_orm::DatabaseConnection;
use tower_cookies::{CookieManagerLayer, Cookies};
use crate::controllers::payment_controller::PaymentController;
use crate::repository::payment_repository::DBPaymentRepository;
use crate::router::helpers::payment_router_helper;
use crate::services::payment_service::PaymentService;

pub fn build_payment_service(connection: &Arc<DatabaseConnection>) -> PaymentService {
    let repository = DBPaymentRepository::new(Arc::clone(connection));
    PaymentService::new(repository)
}

fn build_controller(connection: &Arc<DatabaseConnection>) -> Arc<PaymentController> {
    let service = build_payment_service(connection);
    Arc::new(PaymentController::new(service))
}

pub fn setup_router(connection: DatabaseConnection) -> anyhow::Result<Router> {
    let connection = Arc::new(connection);

    let cookies = Cookies::default();

    let router = Router::new()
        // .route("/", post(payment_router_helper::make_payment))
        .with_state(build_controller(&connection))
        .layer(CookieManagerLayer::new())
        .layer(Extension(cookies));

    Ok(router)
}