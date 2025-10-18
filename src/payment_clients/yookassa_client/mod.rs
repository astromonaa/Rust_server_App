mod types;

use std::net::IpAddr;
use std::sync::Arc;
use axum::http::HeaderValue;
use reqwest::{Client, Url};
use serde_json::json;
use uuid::Uuid;
use base64::{engine::general_purpose, Engine as _};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use crate::payment_clients::yookassa_client::types::CreatePaymentResponse;


pub struct YooKassa {
    client: Arc<Client>,
    pub provider: String,
    base_url: Url,
    allowed_list: [&'static str; 7],
}

impl YooKassa {
    pub fn new(shop_id: String, secret_key: String) -> Self {

        let auth_value = format!("{}:{}", shop_id, secret_key);
        let encoded = general_purpose::STANDARD.encode(auth_value);

        let base_url = Url::parse("https://api.yookassa.ru/v3/").expect("invalid base URL");

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Basic {}", encoded)).unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = Client::builder()
            .default_headers(headers)
            .user_agent("MyApp-YooKassa-Client/1.0")
            .build()
            .unwrap();

        Self {
            client: Arc::new(client),
            provider: "YooKassa".to_string(),
            base_url,
            allowed_list: [
                "185.71.76.0/27",
                "185.71.77.0/27",
                "77.75.153.0/25",
                "77.75.156.11/32",
                "77.75.156.35/32",
                "77.75.154.128/25",
                "2a02:5180::/32",
            ]
        }
    }

    pub async fn create_payment(&self, amount: f64, currency: String, order_id: i32, user_id: i32) -> anyhow::Result<CreatePaymentResponse> {
        let payment_request = json!({
            "amount": { "value": format!("{:.2}", amount), "currency": currency },
            "confirmation": { "type": "redirect", "return_url": "https://google.com" },
            "capture": true,
            "description": format!("Оплата заказа #{}", order_id),
            "metadata": {
                "orderId": order_id,
                "userId": user_id
            }
        });

        let url = self.base_url.join("payments")?;

        let response: CreatePaymentResponse = self.client
            .post(url)
            .header("Idempotence-Key", &Uuid::new_v4().to_string())
            .body(serde_json::to_string(&payment_request)?)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    pub fn is_allowed_id(&self, _: IpAddr) -> bool {
        true
        // self.allowed_list.iter().any(|allowed_ip| {
        //     allowed_ip.parse::<IpNet>().map(|n| n.contains(&ip)).unwrap_or(false)
        // })
    }
}












