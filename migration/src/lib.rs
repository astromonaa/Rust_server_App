pub use sea_orm_migration::prelude::*;

mod m20251012_164617_create_orders_table;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251012_164617_create_orders_table::Migration),
        ]
    }
}
