use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "Users", rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    #[sea_orm(column_type = "Text", unique)]
    pub email: String,

    #[sea_orm(column_type = "Text")]
    pub password: String,

    #[sea_orm(column_type = "Text", default_value="USER")]
    pub role: String,

    #[sea_orm(column_type = "Text")]
    pub activation_link: String,

    #[sea_orm(column_type = "Boolean", default_value=false)]
    pub is_activated: bool,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: chrono::DateTime<Utc>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::cart::Entity")]
    Cart,

    #[sea_orm(has_one = "super::favorite::Entity")]
    Favorite,

    #[sea_orm(has_one = "super::token::Entity")]
    Token,

    #[sea_orm(has_many = "super::order::Entity")]
    Order,
}


impl Related<super::cart::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Cart.def()
    }
}

impl Related<super::favorite::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Favorite.def()
    }
}

impl Related<super::token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Token.def()
    }
}

impl Related<super::order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}