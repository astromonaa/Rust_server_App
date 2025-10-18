use std::sync::Arc;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, Set, Condition, EntityTrait, QueryFilter, DbErr};
use crate::db::entities::payment::{ActiveModel, PaymentStatus, Model, Column, Entity};

pub struct TransactionData {
    pub order_id: i32,
    pub amount: f64,
    pub provider: String,
    pub transaction_id: String,
    pub currency: String,
    pub raw_response: Option<String>,
}

pub struct DBPaymentRepository {
    connection: Arc<DatabaseConnection>,
}

impl DBPaymentRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> DBPaymentRepository {
        DBPaymentRepository { connection }
    }

    pub async fn create_transaction(&self, data: TransactionData, txn: &DatabaseTransaction) -> anyhow::Result<Model> {
        let transaction = ActiveModel {
            amount: Set(data.amount),
            order_id: Set(data.order_id),
            transaction_id: Set(data.transaction_id),
            provider: Set(data.provider),
            raw_response: Set(data.raw_response),
            currency: Set(data.currency),
            status: Set(PaymentStatus::Pending),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        let transaction = transaction.insert(txn).await?;
        Ok(transaction)
    }

    pub async fn update_payment_to_success(&self, txn: &DatabaseTransaction, transaction_id: &String) -> anyhow::Result<Option<()>> {
        let mut condition = Condition::all();
        condition = condition.add(Column::TransactionId.eq(transaction_id));

        let payment = Entity::find()
            .filter(condition)
            .one(txn)
            .await?;

        if payment.is_none() {
            return Ok(None);
        }

        let payment = payment.unwrap();

        let mut active: ActiveModel = payment.into();

        active.status = Set(PaymentStatus::Success);
        active.updated_at = Set(Utc::now());

        active.update(txn).await?;

        Ok(Some(()))
    }

    pub async fn get_user_payments(&self, order_ids: Vec<i32>) -> Result<Vec<Model>, DbErr> {
        let payments = Entity::find()
            .filter(Column::OrderId.is_in(order_ids))
            .all(&*self.connection)
            .await?;

        Ok(payments)
    }
}