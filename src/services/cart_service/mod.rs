pub mod types;

use crate::repository::cart_repository::DBCartRepository;
use crate::db::entities::cart::{ Model as CartModel};
use anyhow::Result;
use sea_orm::DeleteResult;
use crate::services::tokens_service::types::UserClaims;
use crate::db::entities::cart_product::{Model as CartProductModel};
use crate::services::cart_service::types::{CartErrors, CartResult};

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

    pub async fn get_user_cart(&self, user_id: i32) -> anyhow::Result<Option<CartModel>> {
        Ok(self.repository.get_cart(user_id).await?)
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
            let product = cart_product.unwrap();
            let new_quantity = product.quantity + 1;
            cart_product = self.repository.change_quantity(product, new_quantity).await?;
        } else {
            cart_product = self.repository.add_to_cart(user_cart.id, product_id, color, size).await?;
        }
        Ok(cart_product.unwrap())
    }

    pub async fn update_cart_product_quantity(&self, _user: Option<UserClaims>, _product_id: i32, _quantity: i32) -> Result<()> {
        Ok(())
    }

    pub async fn get_user_cart_products(&self, cart_id: i32) -> Result<Vec<CartProductModel>> {
        Ok(self.repository.get_user_cart_products(cart_id).await?)
    }


    pub async fn get_product_ids_in_cart(&self, user_id: i32) -> Vec<i32> {
        let cart = self.repository.get_cart(user_id).await.unwrap();
        self.repository.get_product_ids_in_cart(cart.unwrap().id).await
    }

    pub async fn delete_from_cart(&self, product_id: i32, user_id: i32) -> Result<CartResult, CartErrors> {
        let user_cart = self.get_user_cart(user_id)
            .await
            .map_err(|e| CartErrors::GettingCartError(e.to_string()))?;

        if user_cart.is_none() {
            return Err(CartErrors::CartNotFoundError);
        }

        let user_cart = user_cart.unwrap();

        let cart_product = self.repository.get_cart_product(user_cart.id, product_id)
            .await
            .map_err(|e| CartErrors::GettingCartError(e.to_string()))?;

        if cart_product.is_none() {
            return Err(CartErrors::CartProductNotFoundError);
        }

        self.repository.delete_from_cart(cart_product.unwrap().id)
            .await
            .map_err(|e| CartErrors::CartProductDeleteError(e.to_string()))?;

        Ok(CartResult::Removed)
    }
}