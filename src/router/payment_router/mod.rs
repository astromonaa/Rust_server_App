use std::sync::Arc;
use axum::{Extension, Router};
use sea_orm::DatabaseConnection;
use tower_cookies::{CookieManagerLayer, Cookies};
use crate::configuration::payment_config;
use crate::controllers::payment_controller::PaymentController;
use crate::payment_clients::yookassa_client::YooKassa;
use crate::repository::payment_repository::DBPaymentRepository;
use crate::services::payment_service::PaymentService;

pub fn build_payment_service(connection: &Arc<DatabaseConnection>) -> anyhow::Result<PaymentService> {
    let payment_config = payment_config::PaymentConfiguration::load()?;
    let repository = DBPaymentRepository::new(Arc::clone(connection));
    let payment_client = YooKassa::new(payment_config.shop_id, payment_config.secret_key);
    Ok(PaymentService::new(repository, payment_client))
}

fn build_controller(connection: &Arc<DatabaseConnection>) -> anyhow::Result<Arc<PaymentController>> {
    let service = build_payment_service(connection)?;
    Ok(Arc::new(PaymentController::new(service)))
}

pub fn setup_router(connection: DatabaseConnection) -> anyhow::Result<Router> {
    let connection = Arc::new(connection);

    let cookies = Cookies::default();

    let router = Router::new()
        // .route("/", post(payment_router_helper::make_payment))
        .with_state(build_controller(&connection)?)
        .layer(CookieManagerLayer::new())
        .layer(Extension(cookies));

    Ok(router)
}