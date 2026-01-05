use axum::{http::StatusCode, response::{Json, Redirect}};
use tower_cookies::{Cookie};

use crate::db::entities::user::Model;
use crate::db::entities::anonymous_user::{Model as AnonymousUserModel };
use crate::services::user_service::UserService;
use crate::router::helpers::user_router_helper::RegistrationData;
use crate::services::user_service::types::{CreatedUser};
use crate::websocket::ws_helpers::BrowserInfo;

pub struct UserController {
    service: UserService,
}

impl UserController {
    pub fn new(service: UserService) -> Self { Self { service } }

    pub async fn get_users(&self) -> Result<Json<Vec<Model>>, (StatusCode, String)>{
        let users = self.service.get_users()
            .await
            .map_err(|e|(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        Ok(Json(users))
    }

    pub async fn registration(&self, body: RegistrationData) -> Result<Json<CreatedUser>, (StatusCode, String)> {
        let user = self.service.registration(body)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        Ok(Json(user))
    }

    pub async fn activate(&self, activation_link: String) -> Result<Redirect, (StatusCode, String)> {
        self.service.activate(activation_link)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Redirect::permanent("https://google.com"))
    }

    pub async fn login(&self, body: RegistrationData) -> Result<Json<CreatedUser>, (StatusCode, String)> {
        let user = self.service.login(body)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        Ok(Json(user))
    }

    pub async fn auth<'a>(&self, refresh_token: Option<Cookie<'a>>) -> Result<Json<CreatedUser>, (StatusCode, String)> {
        let token_string = refresh_token.ok_or((StatusCode::BAD_REQUEST, "Refresh token not found".to_string()))?.value().to_string();

        let user = self.service.auth(token_string)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        Ok(Json(user))
    }

    pub async fn logout<'a>(&self, refresh_token: Option<Cookie<'a>>) -> Result<Json<String>, (StatusCode, String)> {
        let token_string = refresh_token.ok_or((StatusCode::BAD_REQUEST, "Refresh token not found".to_string()))?.value().to_string();
        let deleted_token = self.service.logout(token_string)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(deleted_token))
    }

    pub async fn create_or_get_anonymous_user(&self, user_data: BrowserInfo) -> Result<Json<AnonymousUserModel>, (StatusCode, String)> {
        let user = self.service.create_or_get_anonymous_user(user_data)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(user))
    }
}