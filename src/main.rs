use tracing::debug;
use rs_cook::bootstrap;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    std::env::set_var("RUST_BACKTRACE", "1");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888").await.unwrap();
    debug!("listening on {:?}", listener);
    let app = bootstrap::create_app().await;
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
