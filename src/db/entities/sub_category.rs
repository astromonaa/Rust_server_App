use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "SubCategories", rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    #[sea_orm(column_type = "Text", unique)]
    pub name: String,
    #[sea_orm(column_type = "Integer", belongs_to = "super::category::Entity", column_name = "CategoryId")]
    pub category_id: i32,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: chrono::DateTime<Utc>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::product::Entity")]
    Product,
    #[sea_orm(belongs_to = "super::category::Entity", from = "Column::CategoryId", to = "super::category::Column::Id")]
    Category,
}


impl Related<super::product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}


impl ActiveModelBehavior for ActiveModel {}