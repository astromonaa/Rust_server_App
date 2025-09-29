use tower::ServiceBuilder;
use tower_http::cors::{Cors, CorsLayer};
use tower_http::services::ServeDir;

pub fn serve_static() -> Cors<ServeDir> {
    let static_content = ServiceBuilder::new()
        .layer(CorsLayer::very_permissive())
        .service(ServeDir::new("static"));

    static_content
}