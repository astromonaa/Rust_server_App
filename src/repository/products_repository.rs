use std::sync::Arc;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};
use sea_orm::prelude::StringLen::N;
use crate::db::entities::product::{Model as ProductModel, Entity as ProductEntity};
use crate::db::entities::{ category::{Entity as CategoryEntity}, sub_category::{Entity as SubCategoryEntity}};
use crate::DTO::product_dto::ProductWithRelations;

pub struct DbProductsRepository {
    pub connection: Arc<DatabaseConnection>,
}

impl DbProductsRepository {

    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }

    pub async fn get_products(&self) -> anyhow::Result<Vec<ProductModel>> {
        let products = ProductEntity::find().all(&*self.connection).await?;
        Ok(products)
    }
    pub async fn get_product_item(&self, product_id: i32) -> anyhow::Result<ProductWithRelations> {
        let product = ProductEntity::find_by_id(product_id)
            .one(&*self.connection)
            .await?.unwrap();

        let category = product.find_related(CategoryEntity)
            .one(&*self.connection)
            .await?.unwrap();


        let sub_category = product.find_related(SubCategoryEntity)
            .one(&*self.connection)
            .await?.unwrap();

        Ok(ProductWithRelations { product, Category: Some(category), SubCategory: Some(sub_category), isInCart: None, favorite: None })
    }
}
