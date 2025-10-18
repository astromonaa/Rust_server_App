use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum PaymentErrors {
    CreatePaymentError(String),
    CreateTransactionError(String),
}

impl fmt::Display for PaymentErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PaymentErrors::CreatePaymentError(msg) => write!(f, "Create payment error {}", msg),
            PaymentErrors::CreateTransactionError(msg) => write!(f, "Create transaction error {}", msg),
        }
    }
}