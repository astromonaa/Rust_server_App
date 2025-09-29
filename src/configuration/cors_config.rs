use tower_http::cors::{CorsLayer};
use axum::http::{Method, header, HeaderValue};
use crate::configuration::global_configuration;
use std::time::Duration;

/// Создаёт CORS middleware с настройками из GlobalConfiguration
pub fn build_cors() -> CorsLayer {
    let config = global_configuration::GlobalConfiguration::load()
        .expect("Failed to load global configuration");

    let origin = HeaderValue::from_str(&config.client_url)
        .expect("CLIENT_URL must be a valid header value");

    CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
            header::COOKIE,
            header::SET_COOKIE,
        ])
        .expose_headers([
            header::SET_COOKIE,
        ])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600))
}
