use std::fmt;
use axum::{response::{IntoResponse, Response}, http::StatusCode};
use serde::Serialize;

#[derive(Debug)]
pub enum RegistrationErrors {
    UsernameOrPassword,
    UserAlreadyExists,
    DatabaseError,
    PasswordHashError,
    TokenGenerationError,
    TokenSaveError,
    CartCreatingError,
    FavoritesCreatingError,
    MessageSendError,
    LinkExpired
}

impl fmt::Display for RegistrationErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegistrationErrors::UserAlreadyExists => write!(f, "User already exists"),
            RegistrationErrors::UsernameOrPassword => write!(f, "Username or password is missing"),
            RegistrationErrors::DatabaseError => write!(f, "Database error"),
            RegistrationErrors::PasswordHashError => write!(f, "Password hash error"),
            RegistrationErrors::TokenGenerationError => write!(f, "Token generation error"),
            RegistrationErrors::TokenSaveError => write!(f, "Token saving error"),
            RegistrationErrors::CartCreatingError => write!(f, "Error when creating cart from database"),
            RegistrationErrors::FavoritesCreatingError => write!(f, "Error when creating favorites"),
            RegistrationErrors::MessageSendError => write!(f, "Error when sending message"),
            RegistrationErrors::LinkExpired => write!(f, "Link expired"),
        }
    }
}


#[derive(Debug)]
pub enum AuthErrors {
    Unauthorized,
    DataNotValid,
    PassCompareError,
    InvalidPassword,
}

impl fmt::Display for AuthErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthErrors::Unauthorized => write!(f, "Unauthorized"),
            AuthErrors::DataNotValid => write!(f, "Data not valid"),
            AuthErrors::PassCompareError => write!(f, "Pass compare error"),
            AuthErrors::InvalidPassword => write!(f, "Invalid password"),
        }
    }
}

impl IntoResponse for AuthErrors {
    fn into_response(self) -> Response {
        match self {
            AuthErrors::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
            AuthErrors::DataNotValid => (StatusCode::BAD_REQUEST, "Invalid token").into_response(),
            AuthErrors::PassCompareError => (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response(),
            AuthErrors::InvalidPassword => (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreatedUser {
    pub id: i32,
    pub role: String,
    pub email: String,
    pub is_activated: bool,
    pub access_token: String,
    pub refresh_token: String,
    pub activation_link: String,
}
