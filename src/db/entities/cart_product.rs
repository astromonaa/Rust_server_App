use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "CartProducts", rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    #[sea_orm(column_type = "Integer")]
    pub quantity: i32,
    #[sea_orm(column_type = "Text")]
    pub color: String,
    #[sea_orm(column_type = "Integer")]
    pub size: i32,
    #[sea_orm(column_type = "Integer", belongs_to = "super::cart::Entity", column_name = "CartId")]
    pub cart_id: i32,  // Внешний ключ на Cart
    #[sea_orm(column_type = "Integer", belongs_to = "super::product::Entity", column_name = "ProductId")]
    pub product_id: i32, // Внешний ключ на Product

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: chrono::DateTime<Utc>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::cart::Entity", from = "Column::CartId", to = "super::cart::Column::Id")]
    Cart,
    #[sea_orm(belongs_to = "super::product::Entity", from = "Column::ProductId", to = "super::product::Column::Id")]
    Product
}


impl Related<super::cart::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Cart.def()
    }
}

impl Related<super::product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}


impl ActiveModelBehavior for ActiveModel {}