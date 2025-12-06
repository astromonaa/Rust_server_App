use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "Chats", rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: chrono::DateTime<Utc>,

    #[sea_orm(column_type = "Boolean")]
    pub is_anonymous: bool,

    #[sea_orm(column_name = "UserId", nullable)]
    pub user_id: Option<i32>,

    #[sea_orm(column_name = "anonymousUserId")]
    pub anonymous_user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,

    #[sea_orm(
        belongs_to = "super::anonymous_user::Entity",
        from = "Column::AnonymousUserId",
        to = "super::anonymous_user::Column::Id"
    )]
    AnonymousUser,

    #[sea_orm(has_many = "super::message::Entity")]
    Message,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::anonymous_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AnonymousUser.def()
    }
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
