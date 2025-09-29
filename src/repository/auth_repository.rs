use sea_orm::{DatabaseConnection};

pub struct DbAuthRepository {
    pub connection: DatabaseConnection,
}

impl DbAuthRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }


}