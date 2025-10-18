use std::collections::HashSet;
use crate::db::entities::{product::Model as ProductModel};
use crate::DTO::product_dto::ProductWithRelations;
use crate::repository::products_repository::DbProductsRepository;
use crate::router::helpers::order_router_helper::OrderItemData;
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

    fn to_product_with_relations(
        &self,
        product: ProductModel,
        user_id: Option<i32>,
        in_cart_ids: &HashSet<i32>,
        in_fav_ids: &HashSet<i32>
    ) -> ProductWithRelations {
        let is_in_cart = user_id.map(|_| in_cart_ids.contains(&product.id));
        let is_in_favorites = user_id.map(|_| in_fav_ids.contains(&product.id));

        ProductWithRelations {
            isInCart: is_in_cart,
            favorite: is_in_favorites,
            Category: None,
            SubCategory: None,
            product,
        }
    }

    pub async fn get_products(&self, user: Option<UserClaims>) -> anyhow::Result<Vec<ProductWithRelations>> {
        let products = self.repository.get_products().await.unwrap_or(Vec::new());

        let user_id = if user.is_some() { Some(user.unwrap().id) } else { None };

        let mut in_cart_ids = HashSet::new();
        let mut in_fav_ids = HashSet::new();


        if let Some(uid) = user_id {
            in_cart_ids = self.cart.get_product_ids_in_cart(uid).await.into_iter().collect();
            in_fav_ids = self.favorites.get_product_ids_in_favorites(uid).await.into_iter().collect();
        }

        let results = products
            .into_iter()
            .map(|product| self.to_product_with_relations(product, user_id, &in_cart_ids, &in_fav_ids))
            .collect::<Vec<_>>();

        Ok(results)
    }

    pub async fn get_product_item(&self, product_id: i32, user: Option<UserClaims>) -> anyhow::Result<Option<ProductWithRelations>> {
        let Some(product) = self.repository.get_product_item(product_id).await? else { return Ok(None) };

        let (is_in_cart, is_in_favorites) = if let Some(ref user) = user {
            let in_cart = self.cart.is_in_cart(user.id, product.product.id).await?;
            let in_favorites = self.favorites.is_in_favorites(user.id, product.product.id).await.unwrap_or(false);
            (in_cart, in_favorites)
        } else {
            (false, false)
        };

        Ok(Some(ProductWithRelations { isInCart: Some(is_in_cart), favorite: Some(is_in_favorites), ..product }))
    }

    pub async fn calculate_products_total_price(&self, items: &Vec<OrderItemData>) -> (f64, Vec<f64>) {
        self.repository.calculate_products_total_price(items).await
    }

    pub async fn get_products_by_ids(&self, ids: Vec<i32>) -> Vec<ProductModel> {
        self.repository.get_products_by_ids(ids).await
    }
}