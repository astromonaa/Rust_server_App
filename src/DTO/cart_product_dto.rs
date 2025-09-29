use std::fmt;
use std::fmt::Debug;
use serde::Serialize;

#[derive(Serialize)]
pub struct CartProductWithRelations {
    #[serde(flatten)]
    pub product: crate::db::entities::product::Model,
    pub quantity: Option<i32>,
    pub size: Option<i32>,
    pub color: Option<String>,
}

#[derive(Serialize)]
pub struct CartProductsWithTotals {
    pub products: Vec<CartProductWithRelations>,
    pub total_price: f64,
}

impl Debug for CartProductWithRelations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CartProductWithRelations")
            .field("product", &self.product)
            .field("quantity", &self.quantity)
            .field("size", &self.size)
            .field("color", &self.color)
            .finish()
    }
}

impl Debug for CartProductsWithTotals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CartProductsWithTotals")
            .field("products", &self.products)
            .field("total_price", &self.total_price)
            .finish()
    }
}