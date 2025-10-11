use std::collections::HashMap;
use crate::repository::cart_repository::DBCartRepository;
use crate::db::entities::cart::{ Model as CartModel};
use anyhow::Result;
use crate::services::tokens_service::types::UserClaims;
use crate::db::entities::cart_product::{Model as CartProductModel};

#[derive(Clone)]
pub struct CartService {
    repository: DBCartRepository,
}

impl CartService {
    pub fn new(repository: DBCartRepository) -> CartService {
        Self {
            repository,
        }
    }

    pub async fn get_user_cart(&self, user_id: i32) -> Result<CartModel> {
        Ok(self.repository.get_cart(user_id).await?.unwrap())
    }

    pub async fn create_cart(&self, user_id: i32) -> Result<CartModel> {
        let new_cart = self.repository.create_cart(user_id).await?;
        Ok(new_cart)
    }

    pub async fn is_in_cart(&self, user_id: i32, product_id: i32) -> Result<bool> {
        let cart = self.repository.get_cart(user_id).await?.unwrap();
        let cart_product = self.repository.get_cart_product(cart.id, product_id).await?;
        Ok(cart_product.is_some())
    }

    pub async fn add_to_cart(&self, user: Option<UserClaims>, product_id: i32, color: String, size: i32) -> Result<CartProductModel> {
        let user_cart = self.repository.get_cart(user.unwrap().id).await?.unwrap();

        let mut cart_product = self.repository.get_cart_product(user_cart.id, product_id).await?;

        if cart_product.is_some() {
            cart_product = self.repository.increase_quantity(cart_product.unwrap()).await?;
        } else {
            cart_product = self.repository.add_to_cart(user_cart.id, product_id, color, size).await?;
        }
        Ok(cart_product.unwrap())
    }

    pub async fn get_user_cart_products(&self, cart_id: i32) -> Result<Vec<CartProductModel>> {
        Ok(self.repository.get_user_cart_products(cart_id).await?)
    }


    pub async fn get_product_ids_in_cart(&self, user_id: i32) -> Vec<i32> {
        let cart = self.repository.get_cart(user_id).await.unwrap();
        self.repository.get_product_ids_in_cart(cart.unwrap().id).await
    }
}