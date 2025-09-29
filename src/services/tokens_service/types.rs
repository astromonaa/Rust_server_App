use std::fmt;
use serde::{Deserialize, Serialize};
use crate::DTO::user_dto::UserDto;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub id: i32,
    pub email: String,
    pub role: String,
    pub is_activated: bool,
    pub exp: usize, // timestamp in seconds
}


impl UserClaims {
    pub fn new(user: &UserDto, exp: usize) -> Self {
        Self {
            id: user.id,
            email: user.email.clone(),
            role: user.role.clone(),
            is_activated: user.is_activated,
            exp,
        }
    }
}

impl Clone for UserClaims {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            exp: self.exp.clone(),
            role: self.role.clone(),
            email: self.email.clone(),
            is_activated: self.is_activated,
        }
    }
}

#[derive(Debug)]
pub enum TokenErrors {
    DatabaseError,
}

impl fmt::Display for TokenErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenErrors::DatabaseError => write!(f, "database error"),
        }
    }
}