use std::sync::Arc;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait, Condition, ColumnTrait, QueryFilter};
use crate::db::entities::product::{Model as ProductModel, Entity as ProductEntity, Column as ProductColumn};
use crate::db::entities::{ category::{Entity as CategoryEntity}, sub_category::{Entity as SubCategoryEntity}};
use crate::DTO::product_dto::ProductWithRelations;
use crate::router::helpers::order_router_helper::OrderItemData;

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

    pub async fn calculate_products_total_price(&self, items: &Vec<OrderItemData>) -> f64 {
        let mut total = 0.0;

        for OrderItemData { product_id, quantity } in items {
            if let Ok(Some(product)) = ProductEntity::find_by_id(*product_id).one(&*self.connection).await {
                total += product.price * *quantity as f64;
            }
        }

        total
    }

    pub async fn get_products_by_ids(&self, ids: Vec<i32>) -> Vec<ProductModel> {
        let mut condition = Condition::all();
        condition = condition
            .add(ProductColumn::Id.is_in(ids));

        ProductEntity::find()
            .filter(condition)
            .all(&*self.connection)
            .await.unwrap()
    }
}
