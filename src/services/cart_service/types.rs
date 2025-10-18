use serde::Serialize;

#[derive(Serialize)]
pub enum CartResult {
    Removed
}

pub enum CartErrors {
    GettingCartError(String),
    CartNotFoundError,
    CartProductNotFoundError,
    CartProductDeleteError(String),
}

impl std::fmt::Display for CartErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CartErrors::GettingCartError(msg) => write!(f, "Error getting user cart {}", msg),
            CartErrors::CartNotFoundError => write!(f, "Cart doesn't exist"),
            CartErrors::CartProductNotFoundError => write!(f, "Cart product doesn't exist"),
            CartErrors::CartProductDeleteError(msg) => write!(f, "Cart product delete error {}", msg),
        }
    }
}

impl std::fmt::Display for CartResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CartResult::Removed => write!(f, "Product successfully removed from cart"),
        }
    }
}