use std::fmt;
use std::fmt::Debug;
use serde::Serialize;
use crate::db::entities::product::{Model as ProductModel};
use crate::db::entities::{category, sub_category };



#[derive(Serialize)]
pub struct ProductWithRelations {
    #[serde(flatten)]
    pub product: ProductModel,
    pub Category: Option<category::Model>,
    pub SubCategory: Option<sub_category::Model>,
    pub isInCart: Option<bool>,
    pub favorite: Option<bool>,
}

impl Debug for ProductWithRelations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProductWithRelations")
            .field("product", &self.product)
            .field("category", &self.Category)
            .field("sub_category", &self.SubCategory)
            .finish()
    }
}