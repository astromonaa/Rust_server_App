use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "Carts", rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    #[sea_orm(column_type = "Integer", column_name = "UserId")]
    pub user_id: i32, // Внешний ключ на User

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: chrono::DateTime<Utc>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::user::Entity", from="Column::UserId", to="super::user::Column::Id")]
    User,
    #[sea_orm(has_many = "super::cart_product::Entity")]
    CartProduct
}

impl Related<super::cart_product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CartProduct.def()
    }
    fn via() -> Option<RelationDef> {
        None
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
    fn via() -> Option<RelationDef> {
        None
    }
}



impl ActiveModelBehavior for ActiveModel {}