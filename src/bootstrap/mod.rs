use std::sync::Arc;
use axum::{Extension, Router};

use crate::repositories::{create_repositories};
use crate::router::router;

pub async fn create_app() -> Router {
    let repositories = Arc::new(create_repositories().await);
    Router::new().nest_service("/api", router()).layer(Extension(repositories))
}