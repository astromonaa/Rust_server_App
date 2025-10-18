use std::collections::HashMap;
use crate::DTO::cart_product_dto::{CartProductWithRelations, CartProductsWithTotals};
use crate::services::cart_service::CartService;
use crate::services::products_service::ProductsService;
use crate::services::tokens_service::types::UserClaims;
use crate::db::entities::cart_product::{Model as CartProductModel};

pub struct CartAndProductService {
    products_service: ProductsService,
    cart_service: CartService
}

impl CartAndProductService {

    pub fn new(cart_service: CartService, products_service: ProductsService) -> CartAndProductService {
        Self {
            cart_service,
            products_service
        }
    }

    pub async fn get_cart(&self, user: Option<UserClaims>) -> anyhow::Result<CartProductsWithTotals> {
        let user_cart = self.cart_service.get_user_cart(user.unwrap().id).await?;

        let cart_products = self.cart_service.get_user_cart_products(user_cart.unwrap().id).await?;
        let products_ids = cart_products.iter().map(|p| p.product_id).collect();
        let products = self.products_service.get_products_by_ids(products_ids).await;

        let cart_product_map: HashMap<i32, CartProductModel> = cart_products
            .into_iter()
            .map(|p| (p.product_id, p))
            .collect();

        let results: Vec<CartProductWithRelations> = products.into_iter().filter_map(|product| {
            cart_product_map.get(&product.id).map(|cp| CartProductWithRelations {
                product,
                color: Some(cp.color.clone()),
                size: Some(cp.size),
                quantity: Some(cp.quantity)
            })
        }).collect();


        let total_price: f64 = results.iter().fold(0.0, |acc, val| acc + val.product.price);

        Ok(CartProductsWithTotals { products: results, total_price })
    }
}