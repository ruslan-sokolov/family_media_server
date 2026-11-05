use axum::{
    routing::get,
    Router,
};

use storage;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World! "}));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:1488").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
