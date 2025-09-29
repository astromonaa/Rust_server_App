pub mod types;

use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use sea_orm::DatabaseConnection;
use crate::configuration::global_configuration::GlobalConfiguration;
use crate::DTO::user_dto::UserDto;
use crate::repository::token_repository::TokenRepository;
use crate::services::tokens_service::types::{UserClaims, TokenErrors};

pub struct TokenService {}

impl TokenService {
    pub fn generate_tokens(user_data: &UserDto) -> Result<(String, String), jsonwebtoken::errors::Error> {
        let access_exp = Utc::now() + Duration::days(1);
        let refresh_exp = Utc::now() + Duration::days(30);
        let global_config = GlobalConfiguration::load().unwrap();

        let access_claim = UserClaims::new(&user_data, access_exp.timestamp() as usize);
        let refresh_claim = UserClaims::new(&user_data, refresh_exp.timestamp() as usize);

        let access_token = jsonwebtoken::encode(
            &Header::default(),
            &access_claim,
            &EncodingKey::from_secret(global_config.access_secret_key.as_bytes()),
        )?;

        let refresh_token = jsonwebtoken::encode(
            &Header::default(),
            &refresh_claim,
            &EncodingKey::from_secret(global_config.refresh_secret_key.as_bytes()),
        )?;

        Ok((access_token, refresh_token))
    }

    pub async fn save_token(refresh_token: String, user_id: i32, connection: &DatabaseConnection) -> Result<String, TokenErrors> {
        let repository = TokenRepository::new(connection);

        let exists_token = repository.get_token(&refresh_token)
            .await
            .map_err(|_| TokenErrors::DatabaseError)?;

        if exists_token.is_some() {
            println!("Token exists");
        }

        let saved_token = repository.save_token(refresh_token, user_id)
            .await
            .map_err(|e| {
                println!("Error: {}", e);
                return TokenErrors::DatabaseError;
            })?;

        Ok(saved_token)
    }

    pub fn validate_refresh_token(refresh_token: &str) -> Option<UserClaims> {
        let global_config = GlobalConfiguration::load().unwrap();

        let key = DecodingKey::from_secret(global_config.refresh_secret_key.as_bytes());
        
        match jsonwebtoken::decode::<UserClaims>(refresh_token, &key, &Validation::default()) {
            Ok(token_data) => Some(token_data.claims),
            Err(_) => None,
        }
    }

    pub fn validate_access_token(access_token: &str) -> Option<UserClaims> {
        let global_config = GlobalConfiguration::load().unwrap();

        let key = DecodingKey::from_secret(global_config.access_secret_key.as_bytes());

        match jsonwebtoken::decode::<UserClaims>(access_token, &key, &Validation::default()) {
            Ok(token_data) => Some(token_data.claims),
            Err(_) => None,
        }
    }

    pub async fn is_token_from_db(refresh_token: &str, connection: &DatabaseConnection) -> Result<bool, TokenErrors> {
        let repository = TokenRepository::new(connection);

        let exists_token = repository.get_token(&refresh_token)
            .await
            .map_err(|_| TokenErrors::DatabaseError)?;

        if exists_token.is_none() {
            return Ok(false);
        }

        Ok(true)
    }

    pub async fn remove_token(refresh_token: &str, connection: &DatabaseConnection) -> Result<String, TokenErrors> {
        let repository = TokenRepository::new(connection);
        let deleted_token = repository.remove_token(refresh_token).await.map_err(|_|TokenErrors::DatabaseError)?;
        Ok(deleted_token)
    }
}