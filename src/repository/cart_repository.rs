use sea_orm::{Set, DatabaseConnection, ActiveModelTrait, Condition, ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use std::sync::Arc;
use crate::db::entities::cart::{ActiveModel, Model as CartModel, Entity as CartEntity, Column as CartColumn};
use crate::db::entities::cart_product::{Model as CartProductModel, Entity as CartProductEntity, Column as CartProductColumn, ActiveModel as CartProductActiveModel};

use anyhow::Result;
use chrono::Utc;

#[derive(Clone)]
pub struct DBCartRepository {
    connection: Arc<DatabaseConnection>,
}

impl DBCartRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> DBCartRepository {
        Self { connection }
    }

    pub async fn create_cart(&self, user_id: i32) -> Result<CartModel> {
        let cart = ActiveModel {
            user_id: Set(user_id),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        let new_cart = cart.insert(&*self.connection).await?;

        Ok(new_cart)
    }


    pub async fn get_cart(&self, user_id: i32) -> Result<Option<CartModel>> {
        let mut condition = Condition::all();
        condition = condition.add(CartColumn::UserId.eq(user_id));

        let cart = CartEntity::find()
            .filter(condition)
            .one(&*self.connection)
            .await?;

        Ok(cart)
    }

    pub async fn get_cart_product(&self, cart_id: i32, product_id: i32) -> Result<Option<CartProductModel>> {
        let mut condition = Condition::all();
        condition = condition
            .add(CartProductColumn::ProductId.eq(product_id))
            .add(CartProductColumn::CartId.eq(cart_id));

        let cart_product = CartProductEntity::find()
            .filter(condition)
            .one(&*self.connection)
            .await?;

        Ok(cart_product)
    }

    pub async fn add_to_cart(&self, cart_id: i32, product_id: i32, color: String, size: i32) -> Result<Option<CartProductModel>> {
        let cart_product = CartProductActiveModel {
            cart_id: Set(cart_id),
            product_id: Set(product_id),
            quantity: Set(1),
            color: Set(color),
            size: Set(size),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        let saved_product = cart_product.insert(&*self.connection)
            .await?;

        Ok(Some(saved_product))
    }

    pub async fn increase_quantity(&self, cart_product: CartProductModel) -> Result<Option<CartProductModel>> {
        let mut active: CartProductActiveModel = cart_product.into();

        active.quantity = Set(active.quantity.unwrap() + 1);

        let updated = active.update(&*self.connection)
            .await?;

        Ok(Some(updated))
    }

    pub async fn get_user_cart_products(&self, cart_id: i32) -> Result<Vec<CartProductModel>> {
        let products = CartProductEntity::find()
            .filter(CartProductColumn::CartId.eq(cart_id))
            .all(&*self.connection)
            .await?;

        Ok(products)
    }

    pub async fn get_product_ids_in_cart(&self, cart_id: i32) -> Vec<i32> {
        let mut condition = Condition::all();
        condition = condition
            .add(CartProductColumn::CartId.eq(cart_id));

        CartProductEntity::find()
            .filter(condition)
            .select_only()
            .column(CartProductColumn::ProductId)
            .into_tuple::<i32>()
            .all(&*self.connection)
            .await
            .unwrap()
    }
}