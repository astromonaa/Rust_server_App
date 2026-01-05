mod router;
mod static_service;
mod controllers;
mod services;
mod db;
mod repository;
mod configuration;
mod DTO;

mod utils;
mod middleware;
mod payment_clients;
mod websocket;

use axum::{ Router};
use anyhow::Result;
use std::net::SocketAddr;
use crate::configuration::auth_configuration;
use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() -> Result<()> {
    let settings = auth_configuration::AuthConfiguration::load()?;
    let connection = db::get_connection_pool(settings).await;

    Migrator::up(&connection, None).await?;

    let router_module = router::setup_router(connection.clone()).await?;
    let ws_module = websocket::setup_ws_module(connection.clone()).await?;

    let cors = configuration::cors_config::build_cors();

    let app = Router::new()
        .nest("/api", router_module)
        .nest("/ws", ws_module)
        .nest_service("/", static_service::serve_static())
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("localhost:5000").await?;
    println!("Listening http://localhost:5000");
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

    Ok(())
}
