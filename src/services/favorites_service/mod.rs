pub mod types;

use crate::repository::favorites_repository::DBFavoritesRepository;
use crate::db::entities::favorite::{Model as FavoriteModel};
use crate::db::entities::favorite_product::{Model as FavoriteProductModel};

use crate::services::favorites_service::types::{FavoriteResult, FavoritesErrors};
use crate::services::tokens_service::types::UserClaims;

#[derive(Clone)]
pub struct FavoritesService {
    repository: DBFavoritesRepository,
}

impl FavoritesService {
    pub fn new(repository: DBFavoritesRepository) -> Self {
        Self {
            repository,
        }
    }

    pub async fn get_user_favorites(&self, user_id: i32) -> Result<FavoriteModel, FavoritesErrors> {
        let model = self.repository.get_favorites(user_id)
            .await
            .map_err(|_| { FavoritesErrors::Unauthorized })?
            .ok_or(FavoritesErrors::Unauthorized)?;

        Ok(model)
    }

    pub async fn get_user_favorite_products(&self, favorites_id: i32) -> anyhow::Result<Vec<FavoriteProductModel>> {
        let list = self.repository.get_user_favorite_products(favorites_id).await?;
        Ok(list)
    }


    pub async fn create_favorites(&self, user_id: i32) -> anyhow::Result<FavoriteModel> {
        let favorites = self.repository.create_favorites(user_id).await?;
        Ok(favorites)
    }


    pub async fn is_in_favorites(&self, user_id: i32, product_id: i32) -> Result<bool, FavoritesErrors> {
        let favorites = self.get_user_favorites(user_id).await?;
        let favorite_product = self.repository
            .get_favorite_product(favorites.id, product_id)
            .await
            .map_err(|_| FavoritesErrors::Unauthorized)?;
        Ok(favorite_product.is_some())
    }

    pub async fn add_to_favorites(&self, user: Option<UserClaims>, product_id: i32) -> Result<FavoriteResult, FavoritesErrors> {
        if user.is_none() { return Err(FavoritesErrors::Unauthorized) }

        let favorites = self.get_user_favorites(user.unwrap().id).await?;

        let exists = self.repository
            .get_favorite_product(favorites.id, product_id)
            .await
            .map_err(|_| { FavoritesErrors::Unauthorized })?;

        if exists.is_some() {
            self.repository
                .remove_from_favorites(exists.unwrap().id)
                .await
                .map_err(|_| { FavoritesErrors::Unauthorized })?;

            return Ok(FavoriteResult::Removed(true));
        }

        let favorite_product = self.repository.add_to_favorites(favorites.id, product_id).await.map_err(|_| {
            FavoritesErrors::Unauthorized
        })?;
        Ok(FavoriteResult::Created(favorite_product))
    }

    pub async fn get_product_ids_in_favorites(&self, user_id: i32) -> Vec<i32> {
        let favorites = self.repository.get_favorites(user_id).await.unwrap();
        self.repository.get_product_ids_in_favorites(favorites.unwrap().id).await
    }

}