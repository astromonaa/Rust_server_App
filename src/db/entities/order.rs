use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use crate::db::entities::payment::PaymentStatus;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum OrderStatus {
    #[sea_orm(string_value = "pending")]
    Pending,       // заказ создан, ждёт оплаты

    #[sea_orm(string_value = "paid")]
    Paid,          // заказ оплачен

    #[sea_orm(string_value = "processing")]
    Processing,    // заказ в обработке (сборка / упаковка)

    #[sea_orm(string_value = "shipped")]
    Shipped,       // заказ передан в доставку

    #[sea_orm(string_value = "delivered")]
    Delivered,     // заказ доставлен

    #[sea_orm(string_value = "canceled")]
    Canceled,      // заказ отменён

    #[sea_orm(string_value = "refunded")]
    Refunded       // заказ возвращён и деньги возвращены
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "Orders", rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    #[sea_orm(column_name = "UserId")]
    pub user_id: i32,

    #[sea_orm(column_type = "Text")]
    pub status: OrderStatus,

    #[sea_orm(column_type = "Float")]
    pub total_amount: f64,

    #[sea_orm(column_type = "Text")]
    pub currency: String,

    #[sea_orm(column_type = "Text")]
    pub payment_status: PaymentStatus,

    #[sea_orm(column_type = "Text")]
    pub payment_method: String,

    #[sea_orm(column_type = "Text")]
    pub shipping_address: String,

    #[sea_orm(column_type = "Text")]
    pub billing_address: String,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: chrono::DateTime<Utc>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: chrono::DateTime<Utc>,

    #[sea_orm(default_expr = "Expr::current_timestamp()", nullable)]
    pub paid_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(default_expr = "Expr::current_timestamp()", nullable)]
    pub canceled_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(column_type = "Text", nullable)]
    pub notes: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(has_many = "super::order_item::Entity")]
    OrderItem,
    #[sea_orm(has_many = "super::payment::Entity")]
    Payment,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::order_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrderItem.def()
    }
}

impl Related<super::payment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Payment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
