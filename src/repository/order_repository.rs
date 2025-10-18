use std::sync::Arc;
use chrono::Utc;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DatabaseTransaction, DbErr, Set, TransactionTrait};
use crate::router::helpers::order_router_helper::{OrderData, OrderItemData};
use crate::db::entities::order::{ActiveModel as OrderActiveModel, Model as OrderModel, Entity as OrderEntity, OrderStatus, Column as OrderColumn};
use crate::db::entities::order_item::{ActiveModel as OrderItemActiveModel, Model as OrderItemModel, Entity as OrderItemEntity, Column as OrderItemColumn};
use crate::db::entities::product::{Entity as ProductEntity, Model as ProductModel};
use crate::db::entities::payment::PaymentStatus;

pub struct DBOrderRepository {
    connection: Arc<DatabaseConnection>,
}

impl DBOrderRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> DBOrderRepository {
        DBOrderRepository { connection }
    }

    pub async fn begin_tx(&self) -> Result<DatabaseTransaction, DbErr> {
        self.connection.begin().await
    }

    pub async fn stop_tx(&self, txn: DatabaseTransaction) -> Result<(), DbErr> {
        Ok(txn.commit().await?)
    }


    pub async fn get_orders(&self, user_id: i32) -> Result<(Vec<OrderModel>, Vec<(OrderItemModel, Vec<ProductModel>)>, Vec<i32>), DbErr> {

        // get orders
        let orders = OrderEntity::find()
            .filter(OrderColumn::UserId.eq(user_id))
            .all(&*self.connection)
            .await?;

        let orders_ids = orders.iter().map(|o| o.id).collect::<Vec<i32>>();

        // get order items
        let order_items = OrderItemEntity::find()
            .filter(OrderItemColumn::OrderId.is_in(orders_ids.clone()))
            .find_with_related(ProductEntity)
            .all(&*self.connection)
            .await?;

        Ok((orders, order_items, orders_ids))
    }

    pub async fn create_order(
        &self,
        user_id: i32,
        _order_data: &OrderData,
        total_amount: f64,
        txn: &DatabaseTransaction
    ) -> Result<OrderModel, DbErr> {
        let new_order = OrderActiveModel {
            user_id: Set(user_id),
            // currency: Set(_order_data.currency.clone()),
            currency: Set(String::from("RUB")),
            canceled_at: Set(None),
            billing_address: Set(_order_data.billing_address.clone()),
            payment_method: Set(_order_data.payment_method.clone()),
            paid_at: Set(None),
            status: Set(OrderStatus::Pending),
            payment_status: Set(PaymentStatus::Pending),
            shipping_address: Set(_order_data.shipping_address.clone()),
            notes: Set(_order_data.notes.clone()),
            total_amount: Set(total_amount),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        let new_order = new_order.insert(txn).await;
        new_order
    }

    pub async fn create_order_products(
        &self,
        items: &Vec<OrderItemData>,
        order_id: i32,
        prices: Vec<f64>,
        txn: &DatabaseTransaction
    ) -> anyhow::Result<()> {
        let models: Vec<_> = items
            .into_iter()
            .enumerate()
            .map(|(idx, item)| {
                let price = *prices.get(idx).unwrap_or(&1.0);
                OrderItemActiveModel {
                    order_id: Set(order_id),
                    product_id: Set(item.product_id),
                    quantity: Set(item.quantity),
                    total: Set(item.quantity as f64 * price),
                    price: Set(price),
                    size: Set(item.size),
                    color: Set(item.color.clone()),
                    created_at: Set(Utc::now()),
                    updated_at: Set(Utc::now()),
                    ..Default::default()
                }
            }).collect();

        OrderItemEntity::insert_many(models).exec(txn).await?;
        Ok(())
    }


    pub async fn update_order_to_success(&self, txn: &DatabaseTransaction, order_id: i32) -> anyhow::Result<Option<()>> {
        let order = OrderEntity::find_by_id(order_id).one(txn).await?;

        if order.is_none() {
            return Ok(None);
        };

        let order = order.unwrap();

        if order.status != OrderStatus::Pending {
            return Ok(None);
        };

        let mut active: OrderActiveModel = order.into();

        active.status = Set(OrderStatus::Paid);
        active.payment_status = Set(PaymentStatus::Success);
        active.paid_at = Set(Some(Utc::now()));
        active.updated_at = Set(Utc::now());

        active.update(txn).await?;

        Ok(Some(()))

    }
}


