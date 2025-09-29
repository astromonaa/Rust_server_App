use futures::future::join_all;
use std::iter::Product;
use crate::db::entities::{product::Model};
use crate::DTO::product_dto::ProductWithRelations;
use crate::repository::products_repository::DbProductsRepository;
use crate::services::cart_service::CartService;
use crate::services::favorites_service::FavoritesService;
use crate::services::tokens_service::types::UserClaims;

pub struct ProductsService {
    repository: DbProductsRepository,
    cart: CartService,
    favorites: FavoritesService,
}

impl ProductsService {
    pub fn new(repository: DbProductsRepository, cart: CartService, favorites: FavoritesService) -> Self {
        Self { repository, cart, favorites }
    }

    async fn to_product_with_relations(&self, product: Model, user_id: Option<i32>) -> anyhow::Result<ProductWithRelations> {
        if user_id.is_none() {
            Ok(ProductWithRelations { isInCart: None, favorite: None, Category: None, SubCategory: None, product })
        } else {
            let is_in_cart = self.cart.is_in_cart(user_id.unwrap(), product.id).await?;
            let is_in_favorites = self.favorites.is_in_favorites(user_id.unwrap(), product.id).await?;
            Ok(ProductWithRelations { isInCart: Some(is_in_cart), favorite: Some(is_in_favorites), Category: None, SubCategory: None, product })
        }
    }

    pub async fn get_products(&self, user: Option<UserClaims>) -> anyhow::Result<Vec<ProductWithRelations>> {
        let products = self.repository.get_products().await?;

        let user_id = if user.is_some() { Some(user.unwrap().id) } else { None };

        let futures = products.into_iter().map(|product| {
            self.to_product_with_relations(product, user_id)
        });

        let results = join_all(futures).await;
        let results: anyhow::Result<Vec<_>> = results.into_iter().collect();

        results
    }

    pub async fn get_product_item(&self, product_id: i32, user: UserClaims) -> anyhow::Result<ProductWithRelations> {
        let product = self.repository.get_product_item(product_id).await?;

        let is_in_cart = self.cart.is_in_cart(user.id, product.product.id).await?;
        let is_in_favorites = self.favorites.is_in_favorites(user.id, product.product.id).await?;

        Ok(ProductWithRelations { isInCart: Some(is_in_cart), favorite: Some(is_in_favorites), ..product })
    }
}