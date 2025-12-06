use axum::extract::Request;
use axum::http::header::USER_AGENT;
use std::net::SocketAddr;

pub struct BrowserInfo {
    pub user_agent: String,
    pub ip_address: String,
    pub device_fingerprint: String,
}

pub fn get_browser_info(request: Request, addr: SocketAddr) -> anyhow::Result<BrowserInfo> {
    let user_agent = request
        .headers()
        .get(USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .unwrap()
        .to_string();

    let ip_address = addr.ip().to_string();

    let device_fingerprint = request
        .uri()
        .query()
        .and_then(|q| {
            url::form_urlencoded::parse(q.as_bytes())
                .find(|(key, _)| key == "fingerprint")
                .map(|(_, value)| value.to_string())
        })
        .ok_or_else(|| anyhow::Error::msg("No fingerprint found"))?;

    Ok(BrowserInfo {
        user_agent,
        ip_address,
        device_fingerprint,
    })
}
