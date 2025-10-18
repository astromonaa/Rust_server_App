use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use sea_orm::prelude::async_trait;
use crate::router::helpers::order_router_helper::OrderNotificationData;
use crate::services::order_service::OrderService;
use crate::services::order_service::types::OrderErrors;


#[async_trait::async_trait]
pub trait PaymentStrategy: Send + Sync {

    async fn process(&self, data: OrderNotificationData, ip: IpAddr) -> Result<(), OrderErrors>;
    fn get_name(&self) -> String;
}

pub struct PaymentSuccess<'a> {
    service: &'a OrderService
}


#[async_trait::async_trait]
impl<'a> PaymentStrategy for PaymentSuccess<'a> {
    async fn process(&self, data: OrderNotificationData, ip: IpAddr) -> Result<(), OrderErrors> {
        self.service.success_payment(data, ip).await
    }

    fn get_name(&self) -> String {
        "PaymentSuccess".to_string()
    }
}

impl<'a> PaymentSuccess<'a> {
    fn new(service: &'a OrderService) -> Self {
        Self { service }
    }
}

pub struct PaymentFailure<'a> {
    service: &'a OrderService
}

#[async_trait::async_trait]
impl<'a> PaymentStrategy for PaymentFailure<'a> {
    async fn process(&self, data: OrderNotificationData, ip: IpAddr) -> Result<(), OrderErrors> {
        self.service.failed_payment(data, ip).await;
        Ok(())
    }
    fn get_name(&self) -> String {
        "PaymentFailure".to_string()
    }
}

impl<'a> PaymentFailure<'a> {
    fn new(service: &'a OrderService) -> Self {
        Self { service }
    }
}

pub struct RefundSuccess<'a> {
    service: &'a OrderService
}


impl<'a> RefundSuccess<'a> {
    fn new(service: &'a OrderService) -> Self {
        Self { service }
    }
    pub async fn process(&self, _data: OrderNotificationData) {
        // Nothing yet
    }
}


pub struct PaymentStrategyRegistry<'a> {
    strategies: HashMap<String, Arc<dyn PaymentStrategy + 'a>>
}

impl<'a> PaymentStrategyRegistry<'a> {
    pub fn new(service: &'a OrderService) -> Self {
        let mut strategies: HashMap<String, Arc<dyn PaymentStrategy + 'a>> = HashMap::new();

        strategies.insert("payment.succeeded".to_string(), Arc::new(PaymentSuccess::new(service)));
        strategies.insert("payment.canceled".to_string(), Arc::new(PaymentSuccess::new(service)));
        strategies.insert("refund.succeeded".to_string(), Arc::new(PaymentSuccess::new(service)));

        Self { strategies }
    }

    pub fn get_strategy(&self, status: &String) -> Arc<dyn PaymentStrategy + 'a> {
        self.strategies.get(status).unwrap().clone()
    }
}