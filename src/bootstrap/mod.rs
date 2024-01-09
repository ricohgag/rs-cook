use std::sync::Arc;
use axum::{Extension, Router};
use axum::http::{header, Method};
use tower_http::cors::{Any, CorsLayer};

use crate::repositories::{create_repositories};
use crate::router::router;


fn cors() -> CorsLayer {
    CorsLayer::new().allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(Any)
}

pub async fn create_app() -> Router {
    let repositories = Arc::new(create_repositories().await);
    Router::new().nest_service("/api", router()).layer(Extension(repositories)).layer(cors())
}