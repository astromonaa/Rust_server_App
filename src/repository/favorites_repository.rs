use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, Condition, EntityTrait, ColumnTrait, QueryFilter, DeleteResult};
use std::sync::Arc;
use crate::db::entities::favorite::{ActiveModel, Model as FavoriteModel, Column as FavoriteColumn, Entity as FavoriteEntity};
use crate::db::entities::favorite_product::{ Model as FavoriteProductModel, Column as FavoriteProductColumn, Entity as FavoriteProductEntity, ActiveModel as FavoriteProductActiveModel };
use crate::db::entities::product::{ Column as ProductColumn, Entity as ProductEntity, Model as ProductModel };
use anyhow::Result;
use chrono::Utc;

pub struct DBFavoritesRepository {
    connection: Arc<DatabaseConnection>,
}

impl DBFavoritesRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }

    pub async fn create_favorites(&self, user_id: i32) -> Result<FavoriteModel> {
        let favorites = ActiveModel {
            user_id: Set(user_id),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        let new_favorites = favorites.insert(&*self.connection).await?;
        Ok(new_favorites)
    }

    pub async fn get_favorites(&self, user_id: i32) -> Result<Option<FavoriteModel>> {
        let mut condition = Condition::all();
        condition = condition.add(FavoriteColumn::UserId.eq(user_id));

        let favorites = FavoriteEntity::find()
            .filter(condition)
            .one(&*self.connection)
            .await?;

        Ok(favorites)
    }

    /* Получаем элементы из таблицы Product на основании ID избранных */
    pub async fn get_product_item_by_favorite_id(&self, product_id: i32) -> Result<ProductModel> {
        let mut condition = Condition::any();
        condition = condition.add(ProductColumn::Id.eq(product_id));
        let product_item = ProductEntity::find()
            .filter(condition)
            .one(&*self.connection)
            .await?
            .unwrap();
        Ok(product_item)
    }

    pub async fn get_favorites_list(&self, favorites_id: i32) -> Result<Vec<FavoriteProductModel>> {
        let mut condition = Condition::all();
        condition = condition.add(FavoriteProductColumn::FavoriteId.eq(favorites_id));
        let favorites = FavoriteProductEntity::find()
            .filter(condition)
            .all(&*self.connection)
            .await?;

        Ok(favorites)
    }

    pub async fn get_favorite_product(&self, favorites_id: i32, product_id: i32) -> Result<Option<FavoriteProductModel>> {
        let mut condition = Condition::all();
        condition = condition
            .add(FavoriteProductColumn::ProductId.eq(product_id))
            .add(FavoriteProductColumn::FavoriteId.eq(favorites_id));

        let favorite_product = FavoriteProductEntity::find()
            .filter(condition)
            .one(&*self.connection)
            .await?;

        Ok(favorite_product)
    }


    pub async fn add_to_favorites(&self, favorite_id: i32, product_id: i32) -> Result<FavoriteProductModel> {
        let favorite = FavoriteProductActiveModel {
            favorite_id: Set(favorite_id),
            product_id: Set(product_id),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        let saved_favorite = favorite.insert(&*self.connection).await?;

        Ok(saved_favorite)
    }

    pub async fn remove_from_favorites(&self, favorite_product_id: i32) -> Result<DeleteResult> {
        let deleted = FavoriteProductEntity::delete_by_id(favorite_product_id)
            .exec(&*self.connection)
            .await?;

        Ok(deleted)
    }

}