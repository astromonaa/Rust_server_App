use std::sync::Arc;
use sea_orm::DatabaseConnection;

pub struct DBPaymentRepository {
    pub connection: Arc<DatabaseConnection>,
}

impl DBPaymentRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> DBPaymentRepository {
        DBPaymentRepository { connection }
    }
}