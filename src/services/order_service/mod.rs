use crate::repository::order_repository::DBOrderRepository;
use crate::router::helpers::order_router_helper::OrderData;
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

    pub async fn create_order(&self, user_id: i32, _order_data: OrderData) -> Option<i32>  {

        let total_amount = self.products_service.calculate_products_total_price(&_order_data.items).await;
        println!("{total_amount}");
        // let order = self.repository.create_order(user_id, _order_data).await;
        // let payment = self.payment_service.make_payment(order.id, user_id).await;

        Some(1)
    }
}

