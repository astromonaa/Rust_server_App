pub mod types;

use crate::db::entities::user::Model;
use crate::db::entities::anonymous_user::{Model as AnonymousUserModel };
use crate::repository::user_repository::{DbUserRepository, UserFilter};
use crate::router::helpers::user_router_helper::RegistrationData;
use crate::DTO::user_dto::UserDto;
use crate::services::tokens_service::TokenService;
use bcrypt::{hash, DEFAULT_COST, verify};
use crate::services::cart_service::CartService;
use crate::services::favorites_service::FavoritesService;
use crate::services::user_service::types::{AuthErrors, CreateAnonymousUser, CreatedUser, RegistrationErrors};
use uuid::Uuid;
use crate::services::mail_service::MailService;
use crate::services::tokens_service::types::TokenErrors;
use crate::websocket::ws_helpers::BrowserInfo;

pub struct UserService {
    repository: DbUserRepository,
    cart: CartService,
    favorites: FavoritesService,
    mail_service: MailService,
}

impl UserService {
    pub fn new(repository: DbUserRepository, cart: CartService, favorites: FavoritesService, mail_service: MailService) -> Self {
        Self { repository, cart, favorites, mail_service }
    }

    pub async fn get_users(&self) -> anyhow::Result<Vec<Model>> {
        self.repository.get_users().await
    }

    async fn tokens_generation_helper(&self, created_user: Model) -> Result<CreatedUser, RegistrationErrors> {
        let user_data: UserDto = created_user.clone().into();

        /* Генерация токенов */
        let (access_token, refresh_token) = TokenService::generate_tokens(&user_data).map_err(|_| RegistrationErrors::TokenGenerationError)?;
        let saved_token = TokenService::save_token(refresh_token, user_data.id, &*self.repository.connection)
            .await
            .map_err(|_| RegistrationErrors::TokenSaveError)?;

        let UserDto { id, email, role, is_activated, activation_link } = user_data;

        Ok(CreatedUser { id, email, role, is_activated, access_token, refresh_token: saved_token, activation_link })
    }

    pub async fn registration(&self, body: RegistrationData) -> Result<CreatedUser, RegistrationErrors> {
        let RegistrationData { email, password } = body;

        if email.is_empty() || password.is_empty() {
            return Err(RegistrationErrors::UsernameOrPassword);
        };

        let candidate = self.repository
            .get_user(UserFilter {
                email: Some(email.clone()),
                ..Default::default()
            })
            .await
            .map_err(|_| RegistrationErrors::DatabaseError)?;

        if candidate.is_some() {
            return Err(RegistrationErrors::UserAlreadyExists);
        };

        let hash_password = hash(password, DEFAULT_COST).map_err(|_| RegistrationErrors::PasswordHashError)?;
        let activation_link = Uuid::new_v4().to_string();

        let created_user = self.repository.create_user(email, hash_password, activation_link).await.map_err(|_| RegistrationErrors::DatabaseError)?;

        /* Создание корзины */
        self.cart.create_cart(created_user.id).await.map_err(|_| RegistrationErrors::CartCreatingError)?;

        /* Создание избранных */
        self.favorites.create_favorites(created_user.id).await.map_err(|_| RegistrationErrors::FavoritesCreatingError)?;

        /* Отправка письма на почту */
        self.mail_service.send_activation_link(&created_user.email, &created_user.activation_link).map_err(|_| RegistrationErrors::MessageSendError)?;

        Ok(self.tokens_generation_helper(created_user).await?)
    }

    pub async fn activate(&self, activation_link: String) -> Result<Model, RegistrationErrors> {
        let candidate = self.repository.get_user(UserFilter {
            activation_link: Some(activation_link),
            ..Default::default()
        })
            .await
            .map_err(|_| RegistrationErrors::DatabaseError)?;

        if candidate.is_none() {
            return Err(RegistrationErrors::LinkExpired);
        }

        let updating_data  = UserFilter {
            is_activated: Some(true),
            ..Default::default()
        };

        let updated = self.repository.update_user(candidate.unwrap().id, updating_data)
            .await
            .map_err(|_| RegistrationErrors::DatabaseError)?;

        Ok(updated)
    }

    pub async fn auth(&self, refresh_token: String) -> Result<CreatedUser, AuthErrors> {
        let token = TokenService::validate_refresh_token(&refresh_token);

        if token.is_none() {
            return Err(AuthErrors::Unauthorized)
        }

        let is_token_from_db = TokenService::is_token_from_db(&refresh_token, &self.repository.connection)
            .await
            .map_err(|_| AuthErrors::Unauthorized)?;

        if !is_token_from_db {
            return Err(AuthErrors::Unauthorized)
        }

        let user = self.repository.get_user(UserFilter {
            id: Some(token.unwrap().id),
            ..Default::default()
        }).await.map_err(|_| AuthErrors::Unauthorized)?.ok_or(AuthErrors::Unauthorized)?;

        Ok(self.tokens_generation_helper(user).await.map_err(|_| AuthErrors::Unauthorized)?)
    }

    pub async fn login(&self, body: RegistrationData) -> Result<CreatedUser, AuthErrors> {
        let RegistrationData { email, password } = body;

        if email.is_empty() || password.is_empty() {
            return Err(AuthErrors::DataNotValid)
        };

        let user = self.repository
            .get_user(UserFilter { email: Some(email), ..Default::default() })
            .await
            .map_err(|_| AuthErrors::Unauthorized)?.ok_or(AuthErrors::Unauthorized)?;

        let is_valid_password = verify(password, &user.password).map_err(|_| AuthErrors::PassCompareError)?;

        if !is_valid_password {
            return Err(AuthErrors::InvalidPassword);
        }

        Ok(self.tokens_generation_helper(user).await.map_err(|_| AuthErrors::Unauthorized)?)
    }

    pub async fn logout(&self, refresh_token: String) -> Result<String, TokenErrors> {
        TokenService::remove_token(&refresh_token, &self.repository.connection).await
    }

    pub async fn create_or_get_anonymous_user(&self, user_data: BrowserInfo) -> Result<AnonymousUserModel, AuthErrors> {
        let candidate = self.repository.get_anonymous_user(&user_data)
        .await
        .map_err(|_| AuthErrors::Unauthorized)?;

        if candidate.is_some() {
            return Ok(candidate.unwrap());
        }

        let user = self.repository.create_anonymous_user(user_data)
        .await
        .map_err(|e| AuthErrors::CreateAnonymousUserError(e.to_string()))?;

        Ok(user)
    }
}