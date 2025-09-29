use sea_orm::{Database, DatabaseConnection};
use crate::configuration::auth_configuration::AuthConfiguration;
pub mod entities;


pub async fn get_connection_pool(config: AuthConfiguration) -> DatabaseConnection {
    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_user,
        config.db_password,
        config.db_host,
        config.db_port,
        config.db_name
    );

    let connection = Database::connect(connection_string).await.unwrap_or_else(|_| panic!("Unable to connect to DB {}:{}",config.db_user, config.db_password));
    connection
}