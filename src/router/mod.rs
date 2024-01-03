pub mod food;
pub mod menu;

use axum::{
    routing::{get},
    Router,
};
use axum::routing::{delete, post, put};

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/food", food_routes())
        .nest("/menu", menu_routes())
}

fn food_routes() -> Router {
    Router::new()
        .route("/page", get(food::select_page))
        .route("/:id", get(food::select_one))
        .route("/", post(food::insert))
        .route("/", put(food::update))
        .route("/:id", delete(food::delete))
}

fn menu_routes() -> Router {
    Router::new()
        .route("/page", get(menu::select_page))
        .route("/:id", get(menu::select_one))
        .route("/", post(menu::insert))
        .route("/", put(menu::update))
        .route("/:id", delete(menu::delete))
}

pub async fn root() -> &'static str {
    "hello world!!!"
}
