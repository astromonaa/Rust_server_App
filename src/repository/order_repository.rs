use std::sync::Arc;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};
use crate::router::helpers::order_router_helper::OrderData;
use crate::db::entities::order::{ActiveModel, Model, OrderStatus};
use crate::db::entities::payment::PaymentStatus;

pub struct DBOrderRepository {
    connection: Arc<DatabaseConnection>,
}

impl DBOrderRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> DBOrderRepository {
        DBOrderRepository { connection }
    }

    pub async fn create_order(&self, user_id: i32, _order_data: OrderData, total_amount: f64) -> Result<Model, DbErr> {
        let new_order = ActiveModel {
            user_id: Set(user_id),
            currency: Set(_order_data.currency),
            canceled_at: Set(None),
            billing_address: Set(_order_data.billing_address),
            payment_method: Set(_order_data.payment_method),
            paid_at: Set(None),
            status: Set(OrderStatus::Pending),
            payment_status: Set(PaymentStatus::Pending),
            shipping_address: Set(_order_data.shipping_address),
            notes: Set(_order_data.notes),
            total_amount: Set(total_amount),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        let new_order = new_order.insert(&*self.connection).await;
        new_order
    }
}


