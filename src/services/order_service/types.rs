use std::fmt;
use std::fmt::Formatter;
use serde::Serialize;
use crate::db::entities::payment::{ Model as PaymentModel };
use crate::db::entities::order::{ Model as OrderModel };
use crate::db::entities::order_item::{ Model as OrderItemModel };
use crate::db::entities::product::{ Model as ProductModel };




#[derive(Debug)]
pub enum OrderErrors {
    OrderCreateDbError(String),
    OrderItemCreateDbError(String),
    StartTransactionError(String),
    StopTransactionError(String),
    PayOrderError(String),
    OrderUpdateError(String),
    ParseError(String),
    GetOrdersError(String),
    GetPaymentsError(String),
}

impl fmt::Display for OrderErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OrderErrors::OrderCreateDbError(msg) => write!(f, "Db Error while creating order {}", msg),
            OrderErrors::StartTransactionError(msg) => write!(f, "Db Error while starting transaction {}", msg),
            OrderErrors::StopTransactionError(msg) => write!(f, "Db Error while stopping transaction {}", msg),
            OrderErrors::OrderItemCreateDbError(msg) => write!(f, "Db Error while creating order item {}", msg),
            OrderErrors::PayOrderError(msg) => write!(f, "Error while paying order {}", msg),
            OrderErrors::OrderUpdateError(msg) => write!(f, "Error while updating order status {}", msg),
            OrderErrors::ParseError(msg) => write!(f, "Parse error: {}", msg),
            OrderErrors::GetOrdersError(msg) => write!(f, "Error while getting orders from DB: {}", msg),
            OrderErrors::GetPaymentsError(msg) => write!(f, "Payment getting error: {}", msg),
        }
    }
}

#[derive(Serialize)]
pub struct OrderWithRelations {
    pub order: OrderModel,
    pub items: Vec<OrderItemWithRelations>,
    pub payment: Option<PaymentModel>,
}

#[derive(Serialize, Debug)]
pub struct OrderItemWithRelations {
    pub id: i32,
    pub quantity: i32,
    pub price: f64,
    pub total: f64,
    pub product_id: i32,
    pub size: i32,
    pub color: String,
    pub name: String,
    pub description: String,
    pub rating: i32,
    pub images: Vec<String>,
}