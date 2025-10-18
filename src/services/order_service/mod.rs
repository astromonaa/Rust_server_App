pub mod types;

use crate::db::entities::payment::{ Model as PaymentModel };
use crate::db::entities::order::{ Model as OrderModel };
use crate::db::entities::order_item::{ Model as OrderItemModel };
use crate::db::entities::product::{ Model as ProductModel };

use std::net::IpAddr;
use crate::repository::order_repository::DBOrderRepository;
use crate::router::helpers::order_router_helper::{OrderData, OrderNotificationData};
use crate::services::order_service::types::{OrderErrors, OrderItemWithRelations, OrderWithRelations};
use crate::services::payment_service::PaymentService;
use crate::services::products_service::ProductsService;

pub struct OrderService {
    repository: DBOrderRepository,
    payment_service: PaymentService,
    products_service: ProductsService
}

impl OrderService {
    pub fn new(
        repository: DBOrderRepository,
        payment_service: PaymentService,
        products_service: ProductsService
    ) -> OrderService {
        OrderService {
            repository,
            payment_service,
            products_service
        }
    }

    fn normalize_order_product(&self, order_product: OrderItemModel, product: Option<&ProductModel>) -> OrderItemWithRelations {
        let product = product.unwrap();
        OrderItemWithRelations {
            id: order_product.id,
            product_id: order_product.product_id,
            quantity: order_product.quantity,
            price: order_product.price,
            total: order_product.total,
            size: order_product.size,
            color: order_product.color,
            name: product.name.clone(),
            description: product.description.clone(),
            rating: product.rating,
            images: product.images.clone(),
        }
    }

    pub fn collect_orders(&self, orders: Vec<OrderModel>, order_items: Vec<(OrderItemModel, Vec<ProductModel>)>, payments: Vec<PaymentModel>) -> Vec<OrderWithRelations> {
        let mut orders_with_relations = Vec::new();
        for order in orders {
            let items = order_items
                .iter()
                .filter(|(item, _)| item.order_id == order.id)
                .cloned()
                .map(|(order_item, product_items)| self.normalize_order_product(order_item, product_items.get(0)))
                .collect::<Vec<_>>();


            let payment = payments
                .iter()
                .find(|payment| payment.order_id == order.id)
                .cloned();

            orders_with_relations.push(OrderWithRelations {
                order,
                payment,
                items
            })
        }

        orders_with_relations
    }

    pub async fn get_user_orders(&self, user_id: i32) -> Result<Vec<OrderWithRelations>, OrderErrors> {
        let (orders, order_items, orders_ids) = self.repository.get_orders(user_id)
            .await
            .map_err(|e| OrderErrors::GetOrdersError(e.to_string()))?;


        let payments = self.payment_service.get_user_payments(orders_ids)
            .await
            .map_err(|e| OrderErrors::GetPaymentsError(e.to_string()))?;

        Ok(self.collect_orders(orders, order_items, payments))

    }

    pub async fn create_order(&self, user_id: i32, _order_data: OrderData) -> Result<Option<String>, OrderErrors>  {

        let txn = self.repository
            .begin_tx()
            .await
            .map_err(|e| OrderErrors::StartTransactionError(e.to_string()))?;

        let (total_amount, prices) = self.products_service.calculate_products_total_price(&_order_data.items).await;

        let order = self.repository
            .create_order(user_id, &_order_data, total_amount, &txn)
            .await
            .map_err(|e|OrderErrors::OrderCreateDbError(e.to_string()))?;

        let _ = self.repository.create_order_products(&_order_data.items, order.id, prices, &txn)
            .await
            .map_err(|e| OrderErrors::OrderItemCreateDbError(e.to_string()))?;

        let confirmation_url = self.payment_service.make_payment(total_amount, &_order_data.currency, order.id, user_id, &txn)
            .await
            .map_err(|e| OrderErrors::PayOrderError(e.to_string()))?;

        self.repository.stop_tx(txn)
            .await
            .map_err(|e| OrderErrors::StopTransactionError(e.to_string()))?;

        Ok(confirmation_url)
    }

    pub async fn success_payment(&self, data: OrderNotificationData, ip: IpAddr) -> Result<(), OrderErrors> {

        if !self.payment_service.is_allowed_ip(ip) { return Ok(()) }

        let txn = self.repository
            .begin_tx()
            .await
            .map_err(|e| OrderErrors::StartTransactionError(e.to_string()))?;

        let order_id = data.object.metadata.order_id.parse::<i32>()
            .map_err(|e| OrderErrors::ParseError(format!("Failed to parse order_id: {}", e)))?;

        let order_result = self.repository.update_order_to_success(&txn, order_id)
            .await
            .map_err(|e| OrderErrors::OrderUpdateError(e.to_string()))?;

        if order_result.is_none() {
            return Ok(())
        }

        self.payment_service.update_payment_to_success(&txn, &data.object.id)
            .await
            .map_err(|e| OrderErrors::OrderUpdateError(e.to_string()))?;

        self.repository.stop_tx(txn)
            .await
            .map_err(|e| OrderErrors::StopTransactionError(e.to_string()))?;

        Ok(())
    }

    pub async fn failed_payment(&self, _data: OrderNotificationData, ip: IpAddr) -> Result<(), OrderErrors> {
        if !self.payment_service.is_allowed_ip(ip) { return Ok(()) }
        Ok(())
    }
}

