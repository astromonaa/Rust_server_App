use std::fmt;
use serde::Serialize;

#[derive(Debug)]
pub enum FavoritesErrors {
    Unauthorized,
    GetProductError
}

impl fmt::Display for FavoritesErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FavoritesErrors::Unauthorized => write!(f, "Unauthorized"),
            FavoritesErrors::GetProductError => write!(f, "Error while getting favorite product"),
        }
    }
}


#[derive(Serialize)]
pub enum FavoriteResult {
    Created(crate::db::entities::favorite_product::Model),
    Removed(bool)
}
