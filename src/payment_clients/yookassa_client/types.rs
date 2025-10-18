use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    pub value: String,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipient {
    pub account_id: String,
    pub gateway_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Confirmation {
    #[serde(rename = "type")]
    pub kind: String,
    pub confirmation_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "orderId")]
    pub order_id: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePaymentResponse {
    pub id: String,
    pub status: String,
    pub amount: Amount,
    pub description: Option<String>,
    pub recipient: Option<Recipient>,
    pub created_at: String,
    pub confirmation: Option<Confirmation>,
    pub test: Option<bool>,
    pub paid: Option<bool>,
    pub refundable: Option<bool>,
    pub metadata: Option<Metadata>,
}