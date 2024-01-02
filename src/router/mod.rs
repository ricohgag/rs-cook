pub mod food;

use axum::{
    routing::{get},
    Router,
};
use axum::routing::{delete, post, put};

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/food", food_routes())
}

fn food_routes() -> Router {
    Router::new()
        .route("/page", get(food::select_page))
        .route("/:id", get(food::select_one))
        .route("/", post(food::insert))
        .route("/", put(food::update))
        .route("/:id", delete(food::delete))
}

pub async fn root() -> &'static str {
    "hello world!!!"
}
