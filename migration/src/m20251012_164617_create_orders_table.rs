use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // ---------- Users ----------
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("Users"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("email")).string().not_null().unique_key())
                    .col(ColumnDef::new(Alias::new("password")).string().not_null())
                    .col(ColumnDef::new(Alias::new("role")).string().not_null().default("USER"))
                    .col(ColumnDef::new(Alias::new("activationLink")).string().not_null())
                    .col(ColumnDef::new(Alias::new("isActivated")).boolean().not_null().default(false))
                    .col(ColumnDef::new(Alias::new("createdAt")).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Alias::new("updatedAt")).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        // ---------- Orders ----------
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("Orders"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("UserId")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("status")).string().not_null())
                    .col(ColumnDef::new(Alias::new("totalAmount")).double().not_null())
                    .col(ColumnDef::new(Alias::new("currency")).string().not_null())
                    .col(ColumnDef::new(Alias::new("paymentStatus")).string().not_null())
                    .col(ColumnDef::new(Alias::new("paymentMethod")).string().not_null())
                    .col(ColumnDef::new(Alias::new("shippingAddress")).string().not_null())
                    .col(ColumnDef::new(Alias::new("billingAddress")).string().not_null())
                    .col(ColumnDef::new(Alias::new("createdAt")).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Alias::new("updatedAt")).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Alias::new("paidAt")).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Alias::new("canceledAt")).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Alias::new("notes")).string().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_orders_user")
                            .from(Alias::new("Orders"), Alias::new("UserId"))
                            .to(Alias::new("Users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // ---------- OrderItems ----------
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("OrderItems"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("OrderId")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("ProductId")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("quantity")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("price")).double().not_null())
                    .col(ColumnDef::new(Alias::new("color")).string().not_null())
                    .col(ColumnDef::new(Alias::new("size")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("total")).double().not_null())
                    .col(ColumnDef::new(Alias::new("createdAt")).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Alias::new("updatedAt")).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_order_items_order")
                            .from(Alias::new("OrderItems"), Alias::new("OrderId"))
                            .to(Alias::new("Orders"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // ---------- Payments ----------
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("Payments"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("OrderId")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("provider")).string().not_null())
                    .col(ColumnDef::new(Alias::new("transactionId")).string().not_null())
                    .col(ColumnDef::new(Alias::new("amount")).double().not_null())
                    .col(ColumnDef::new(Alias::new("currency")).string().not_null())
                    .col(ColumnDef::new(Alias::new("status")).string().not_null())
                    .col(ColumnDef::new(Alias::new("rawResponse")).text().null())
                    .col(ColumnDef::new(Alias::new("createdAt")).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Alias::new("updatedAt")).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payments_order")
                            .from(Alias::new("Payments"), Alias::new("OrderId"))
                            .to(Alias::new("Orders"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Alias::new("Payments")).to_owned()).await?;
        manager.drop_table(Table::drop().table(Alias::new("OrderItems")).to_owned()).await?;
        manager.drop_table(Table::drop().table(Alias::new("Orders")).to_owned()).await?;
        manager.drop_table(Table::drop().table(Alias::new("Users")).to_owned()).await?;
        Ok(())
    }
}
