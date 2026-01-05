use std::sync::Arc;
use axum::{Json, response::IntoResponse, extract::{Path, State}, Extension};
use axum::http::StatusCode;
use serde::Deserialize;
use crate::controllers::user_controller::UserController;
use crate::services::user_service::types::{CreatedUser};
use tower_cookies::{Cookie, Cookies};
use time::Duration;

#[derive(Debug, Clone, Deserialize)]
pub struct RegistrationData {
    pub email: String,
    pub password: String,
}

pub async fn get_users(State(user_controller): State<Arc<UserController>>) -> impl IntoResponse {
    user_controller.get_users().await
}

pub async fn registration(
    State(user_controller): State<Arc<UserController>>,
    Json(body): Json<RegistrationData>
) -> impl IntoResponse {
    user_controller.registration(body).await
}

pub async fn login(
    State(user_controller): State<Arc<UserController>>,
    Extension(cookies): Extension<Cookies>,
    Json(body): Json<RegistrationData>,
) -> Result<Json<CreatedUser>, (StatusCode, String)> {

    let user = user_controller.login(body).await?;


    let mut updated_token = Cookie::new("refresh_token", user.refresh_token.to_string());
    updated_token.set_http_only(true);
    updated_token.set_max_age(Duration::days(30));
    cookies.add(updated_token);

    Ok(user)
}

pub async fn auth(
    State(user_controller): State<Arc<UserController>>,
    Extension(cookies): Extension<Cookies>,
) -> Result<Json<CreatedUser>, (StatusCode, String)> {
    let refresh_token = cookies.get("refresh_token");

    let user = user_controller.auth(refresh_token).await?;

    let mut updated_token = Cookie::new("refresh_token", user.refresh_token.to_string());
    updated_token.set_http_only(true);
    updated_token.set_max_age(Duration::days(30));
    cookies.add(updated_token);

    Ok(user)
}

pub async fn activate_user(
    State(user_controller): State<Arc<UserController>>,
    Path(activation_link): Path<String>,
) -> impl IntoResponse {
    user_controller.activate(activation_link).await
}

pub async fn logout(
    State(user_controller): State<Arc<UserController>>,
    Extension(cookies): Extension<Cookies>,
) -> impl IntoResponse {
    let refresh_token = cookies.get("refresh_token");
    user_controller.logout(refresh_token).await
}