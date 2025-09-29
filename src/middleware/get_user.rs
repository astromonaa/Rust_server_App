use axum::{
    http::{Request, header},
    response::{Response},
    body::Body,
};
use crate::services::tokens_service::{TokenService};
use crate::services::tokens_service::types::UserClaims;

pub async fn get_user_middleware(mut req: Request<Body>, next: axum::middleware::Next) -> Response {

    if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            let mut parts = auth_str.split_whitespace();
            if let (Some(_), Some(token)) = (parts.next(), parts.next()) {
                if token != "null" && token != "undefined" {
                    let user = TokenService::validate_access_token(token);
                    if user.is_some() {
                        req.extensions_mut().insert(user);
                        return next.run(req).await;
                    }
                }
            }
        }

    }
    req.extensions_mut().insert(None::<UserClaims>);
    next.run(req).await
}