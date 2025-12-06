use axum::{
    http::{Request, StatusCode},
    response::Response,
    body::Body,
};
use std::collections::HashMap;
use crate::services::tokens_service::TokenService;
use crate::services::tokens_service::types::UserClaims;

pub async fn get_user_middleware_ws(mut req: Request<Body>, next: axum::middleware::Next) -> Response {
    // Extract token from query parameters
    if let Some(query) = req.uri().query() {
        let params: HashMap<_, _> = url::form_urlencoded::parse(query.as_bytes())
            .into_owned()
            .collect();

        if let Some(token) = params.get("token") {
            if token != "null" && token != "undefined" {
                let user = TokenService::validate_access_token(token);
                if user.is_some() {
                    req.extensions_mut().insert(user);
                    return next.run(req).await;
                }
            }
        }
    }

    req.extensions_mut().insert(None::<UserClaims>);
    next.run(req).await
}