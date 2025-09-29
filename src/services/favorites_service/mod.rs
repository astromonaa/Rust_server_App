use futures::future::join_all;
pub mod types;

use crate::repository::favorites_repository::DBFavoritesRepository;
use crate::db::entities::favorite::{Model as FavoriteModel};
use crate::db::entities::product::{Model as ProductModel};

use crate::services::favorites_service::types::{FavoriteResult, FavoritesErrors};
use crate::services::tokens_service::types::UserClaims;

pub struct FavoritesService {
    repository: DBFavoritesRepository
}

impl FavoritesService {
    pub fn new(repository: DBFavoritesRepository) -> Self {
        Self {
            repository
        }
    }

    pub async fn get_favorites_list(&self, user_id: i32) -> anyhow::Result<Vec<ProductModel>> {
        let favorites = self.repository.get_favorites(user_id).await?.unwrap();
        let favorites_list_promises = self.repository.get_favorites_list(favorites.id)
            .await?
            .into_iter()
            .map(|favorite_item| self.repository.get_product_item_by_favorite_id(favorite_item.product_id));

        let results = join_all(favorites_list_promises).await;
        let results = results.into_iter().collect::<Result<Vec<_>, _>>()?;
        Ok(results)
    }

    pub async fn create_favorites(&self, user_id: i32) -> anyhow::Result<FavoriteModel> {
        let favorites = self.repository.create_favorites(user_id).await?;
        Ok(favorites)
    }


    pub async fn is_in_favorites(&self, user_id: i32, product_id: i32) -> anyhow::Result<bool> {
        let favorites = self.repository.get_favorites(user_id).await?.unwrap();
        let favorite_product = self.repository.get_favorite_product(favorites.id, product_id).await?;
        Ok(favorite_product.is_some())
    }

    pub async fn add_to_favorites(&self, user: Option<UserClaims>, product_id: i32) -> Result<FavoriteResult, FavoritesErrors> {
        if user.is_none() { return Err(FavoritesErrors::Unauthorized) }

        let favorites = self.repository.get_favorites(user.unwrap().id)
            .await
            .map_err(|_| { FavoritesErrors::Unauthorized })?
            .ok_or(FavoritesErrors::Unauthorized)?;

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

        let favorite_product = self.repository.add_to_favorites(favorites.id, product_id).await.map_err(|e| {
            println!("favorites: {:#?}", e);
            FavoritesErrors::Unauthorized
        })?;
        Ok(FavoriteResult::Created(favorite_product))
    }

}