mod types;
use crate::db::entities::payment::{Model};

use std::net::IpAddr;
use sea_orm::{DatabaseTransaction, DbErr};
use crate::payment_clients::yookassa_client::YooKassa;
use crate::repository::payment_repository::{DBPaymentRepository, TransactionData};
use crate::services::payment_service::types::PaymentErrors;

pub struct PaymentService {
    repository: DBPaymentRepository,
    payment_client: YooKassa
}

impl PaymentService {
    pub fn new(repository: DBPaymentRepository, payment_client: YooKassa) -> PaymentService {
        PaymentService {
            repository,
            payment_client
        }
    }

    pub async fn make_payment(&self, amount: f64, currency: &str, order_id: i32, user_id: i32, txn: &DatabaseTransaction) -> Result<Option<String>, PaymentErrors> {
        let payment = self.payment_client
            .create_payment(amount, currency.to_string(), order_id, user_id)
            .await
            .map_err(|e| PaymentErrors::CreatePaymentError(e.to_string()))?;

        let provider = &self.payment_client.provider;

        let transaction_data = TransactionData {
            amount,
            order_id,
            currency: currency.to_string(),
            provider: provider.clone(),
            transaction_id: payment.id,
            raw_response: None,
        };
        let _ = self.repository.create_transaction(transaction_data, txn)
            .await
            .map_err(|e| PaymentErrors::CreateTransactionError(e.to_string()))?;

        Ok(payment.confirmation.unwrap().confirmation_url)
    }

    pub async fn update_payment_to_success(&self, txn: &DatabaseTransaction, transaction_id: &String) -> anyhow::Result<Option<()>> {
        self.repository.update_payment_to_success(txn, transaction_id).await
    }

    pub async fn get_user_payments(&self, order_ids: Vec<i32>) -> Result<Vec<Model>, DbErr> {
        self.repository.get_user_payments(order_ids).await
    }

    pub fn is_allowed_ip(&self, ip: IpAddr) -> bool {
        self.payment_client.is_allowed_id(ip)
    }
}