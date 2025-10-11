use crate::repository::payment_repository::DBPaymentRepository;

pub struct PaymentService {
    repository: DBPaymentRepository
}

impl PaymentService {
    pub fn new(repository: DBPaymentRepository) -> PaymentService {
        PaymentService {
            repository
        }
    }
}