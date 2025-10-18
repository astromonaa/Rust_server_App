use std::sync::Arc;
use axum::{Extension, Router};
use axum::routing::{get, post};
use axum::middleware::from_fn;
use sea_orm::DatabaseConnection;
use tower_cookies::{CookieManagerLayer, Cookies};
use crate::controllers::order_controller::OrderController;
use crate::repository::order_repository::DBOrderRepository;
use crate::router::helpers::order_router_helper;
use crate::router::{build_product_service, payment_router};
use crate::services::order_service::OrderService;
use crate::middleware::get_user::get_user_middleware;

fn build_controller(connection: &Arc<DatabaseConnection>) -> anyhow::Result<Arc<OrderController>> {
    let order_repository = DBOrderRepository::new(Arc::clone(connection));
    let payment_service = payment_router::build_payment_service(connection)?;
    let product_service = build_product_service(connection);
    let order_service = OrderService::new(order_repository, payment_service, product_service);
    Ok(Arc::new(OrderController::new(order_service)))
}

pub fn setup_router(connection: DatabaseConnection) -> anyhow::Result<Router> {
    let connection = Arc::new(connection);

    let cookies = Cookies::default();

    let router = Router::new()
        .route("/create", post(order_router_helper::create_order).route_layer(from_fn(get_user_middleware)))
        .route("/notify", post(order_router_helper::update_order_status).route_layer(from_fn(get_user_middleware)))
        .route("/", get(order_router_helper::get_orders).route_layer(from_fn(get_user_middleware)))
        .route("/:order_id", get(order_router_helper::get_order_by_id).route_layer(from_fn(get_user_middleware)))
        .with_state(build_controller(&connection)?)
        .layer(CookieManagerLayer::new())
        .layer(Extension(cookies));

    Ok(router)
}
