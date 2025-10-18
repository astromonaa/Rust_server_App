use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "Products", rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub rating: i32,

    #[sea_orm(column_type = "Json")]
    pub images: Vec<String>,

    #[sea_orm(column_type = "Json")]
    pub sizes: Vec<i32>,

    #[sea_orm(column_type = "Json")]
    pub colors: Vec<String>,

    #[sea_orm(column_name = "CategoryId")]
    pub category_id: i32,

    #[sea_orm(column_name = "SubCategoryId")]
    pub sub_category_id: i32,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: chrono::DateTime<Utc>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::cart_product::Entity")]
    CartProduct,

    #[sea_orm(has_many = "super::favorite_product::Entity")]
    FavoriteProduct,

    #[sea_orm(belongs_to = "super::category::Entity", from="Column::CategoryId", to="super::category::Column::Id")]
    Category,

    #[sea_orm(belongs_to = "super::sub_category::Entity", from="Column::SubCategoryId", to="super::sub_category::Column::Id")]
    SubCategory,

    #[sea_orm(has_many = "super::order_item::Entity")]
    OrderItem,
}

impl Related<super::cart_product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CartProduct.def()
    }
}

impl Related<super::favorite_product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FavoriteProduct.def()
    }
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<super::sub_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubCategory.def()
    }
}

impl Related<super::order_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrderItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
